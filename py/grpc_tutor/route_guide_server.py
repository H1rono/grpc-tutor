import logging
import math
import time
from collections.abc import Iterator
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from typing import Self

import grpc
from generated import route_guide_pb2, route_guide_pb2_grpc


@dataclass(frozen=True)
class Point:
    latitude: int
    longitude: int

    @classmethod
    def from_pb2_point(cls, value: route_guide_pb2.Point) -> Self:
        return cls(value.latitude, value.longitude)

    def into_pb2_point(self) -> route_guide_pb2.Point:
        return route_guide_pb2.Point(latitude=self.latitude, longitude=self.longitude)

    def distance_of(self, other: Self) -> float:
        # https://github.com/grpc/grpc/blob/b8a04ac/examples/python/route_guide/route_guide_server.py#L35-L56
        """Distance between two points."""
        coord_factor = 10000000.0
        lat_1 = self.latitude / coord_factor
        lat_2 = other.latitude / coord_factor
        lon_1 = self.longitude / coord_factor
        lon_2 = other.longitude / coord_factor
        lat_rad_1 = math.radians(lat_1)
        lat_rad_2 = math.radians(lat_2)
        delta_lat_rad = math.radians(lat_2 - lat_1)
        delta_lon_rad = math.radians(lon_2 - lon_1)

        # Formula is based on http://mathforum.org/library/drmath/view/51879.html
        a = pow(math.sin(delta_lat_rad / 2), 2) + (
            math.cos(lat_rad_1)
            * math.cos(lat_rad_2)
            * pow(math.sin(delta_lon_rad / 2), 2)
        )
        c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))
        R = 6371000
        # metres
        return R * c


@dataclass(frozen=True)
class Rectangle:
    lo: Point
    hi: Point

    @classmethod
    def from_pb2_rectangle(cls, value: route_guide_pb2.Rectangle) -> Self:
        return cls(Point.from_pb2_point(value.lo), Point.from_pb2_point(value.hi))

    def into_pb2_rectangle(self) -> route_guide_pb2.Rectangle:
        return route_guide_pb2.Rectangle(
            lo=self.lo.into_pb2_point(), hi=self.hi.into_pb2_point()
        )

    def includes_latitude(self, latitude: int) -> bool:
        latitude_min = min(self.lo.latitude, self.hi.latitude)
        latitude_max = max(self.lo.latitude, self.hi.latitude)
        return latitude_min <= latitude <= latitude_max

    def includes_longitude(self, longitude: int) -> bool:
        longitude_min = min(self.lo.longitude, self.hi.longitude)
        longitude_max = max(self.lo.longitude, self.hi.longitude)
        return longitude_min <= longitude <= longitude_max

    def includes_point(self, point: Point) -> bool:
        include_latitude = self.includes_latitude(point.latitude)
        include_longitude = self.includes_longitude(point.longitude)
        return include_latitude and include_longitude


class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
    def __init__(self) -> None:
        super().__init__()
        # 35.681042, 139.767214
        self.db = {Point(356810420, 1397672140): "tokyo"}

    def GetFeature(
        self,
        request: route_guide_pb2.Point,
        _context: route_guide_pb2_grpc._ServicerContext,
    ) -> route_guide_pb2.Feature:
        position = Point.from_pb2_point(request)
        name = self.db.get(position, "")
        return route_guide_pb2.Feature(name=name, location=position.into_pb2_point())

    def ListFeatures(
        self,
        request: route_guide_pb2.Rectangle,
        _context: route_guide_pb2_grpc._ServicerContext,
    ) -> Iterator[route_guide_pb2.Feature]:
        rectangle = Rectangle.from_pb2_rectangle(request)
        return (
            route_guide_pb2.Feature(name=self.db[point], location=point.into_pb2_point())
            for point in self.db
            if rectangle.includes_point(point)
        )

    def RecordRoute(
        self,
        request_iterator: route_guide_pb2_grpc._MaybeAsyncIterator[route_guide_pb2.Point],
        context: route_guide_pb2_grpc._ServicerContext,
    ) -> route_guide_pb2.RouteSummary:
        count = 0
        feature_count = 0
        distance = 0.0
        start = time.time()
        prev_position: Point | None = None
        for position in request_iterator:
            pos = Point.from_pb2_point(position)
            if prev_position is not None:
                distance += prev_position.distance_of(pos)
            prev_position = pos
            feature = self.GetFeature(position, context)
            if feature.name:
                feature_count += 1
            count += 1
        end = time.time()
        return route_guide_pb2.RouteSummary(
            point_count=count,
            feature_count=feature_count,
            distance=int(distance),
            elapsed_time=int(end - start),
        )

    def RouteChat(
        self,
        request_iterator: route_guide_pb2_grpc._MaybeAsyncIterator[
            route_guide_pb2.RouteNote
        ],
        _context: route_guide_pb2_grpc._ServicerContext,
    ) -> Iterator[route_guide_pb2.RouteNote]:
        # https://github.com/grpc/grpc/blob/b8a04ac/examples/python/route_guide/route_guide_server.py#L109-L115
        prev_notes: list[route_guide_pb2.RouteNote] = []
        for new_note in request_iterator:
            for prev_note in prev_notes:
                if prev_note.location == new_note.location:
                    yield prev_note
            prev_notes.append(new_note)


def serve() -> None:
    with ThreadPoolExecutor(max_workers=10) as executor:
        server = grpc.server(executor)
        route_guide_pb2_grpc.add_RouteGuideServicer_to_server(
            RouteGuideServicer(), server
        )
        server.add_insecure_port("[::]:50051")
        server.start()
        server.wait_for_termination()


if __name__ == "__main__":
    logging.basicConfig()
    serve()

import logging
import time
from collections.abc import Iterator
from concurrent.futures import ThreadPoolExecutor

import grpc
from generated import route_guide_pb2, route_guide_pb2_grpc

from .route_guide_db import Point, Rectangle, load_db


class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
    def __init__(self) -> None:
        super().__init__()
        self.db = {f.location: f.name for f in load_db()}

    def GetFeature(
        self,
        request: route_guide_pb2.Point,
        _context: grpc.ServicerContext,
    ) -> route_guide_pb2.Feature:
        position = Point.from_pb2_point(request)
        name = self.db.get(position, "")
        return route_guide_pb2.Feature(name=name, location=position.into_pb2_point())

    def ListFeatures(
        self,
        request: route_guide_pb2.Rectangle,
        _context: grpc.ServicerContext,
    ) -> Iterator[route_guide_pb2.Feature]:
        rectangle = Rectangle.from_pb2_rectangle(request)
        return (
            route_guide_pb2.Feature(name=self.db[point], location=point.into_pb2_point())
            for point in self.db
            if rectangle.includes_point(point)
        )

    def RecordRoute(
        self,
        request_iterator: Iterator[route_guide_pb2.Point],
        context: grpc.ServicerContext,
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
        request_iterator: Iterator[route_guide_pb2.RouteNote],
        _context: grpc.ServicerContext,
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

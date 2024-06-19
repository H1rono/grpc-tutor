import math
from dataclasses import dataclass
from typing import Self

from generated import route_guide_pb2


@dataclass(frozen=True)
class Point:
    latitude: int
    longitude: int

    @classmethod
    def from_pb2_point(cls, value: route_guide_pb2.Point) -> Self:
        return cls(value.latitude, value.longitude)

    def into_pb2_point(self) -> route_guide_pb2.Point:
        return route_guide_pb2.Point(latitude=self.latitude, longitude=self.longitude)

    @classmethod
    def from_dict(cls, value: dict) -> Self:
        return cls(value["latitude"], value["longitude"])

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
class Feature:
    name: str
    location: Point

    @classmethod
    def from_pb2_feature(cls, value: route_guide_pb2.Feature) -> Self:
        name = value.name
        location = Point.from_pb2_point(value.location)
        return cls(name, location)

    def into_pb2_feature(self) -> route_guide_pb2.Feature:
        return route_guide_pb2.Feature(
            name=self.name, location=self.location.into_pb2_point()
        )

    @classmethod
    def from_dict(cls, value: dict) -> Self:
        name = value["name"]
        location = Point.from_dict(value["location"])
        return cls(name, location)


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

import logging
import random

import grpc
from generated import route_guide_pb2, route_guide_pb2_grpc

from .route_guide_db import load_db


def get_feature(stub: route_guide_pb2_grpc.RouteGuideStub) -> None:
    feature = stub.GetFeature(
        route_guide_pb2.Point(latitude=356810420, longitude=1397672140)
    )
    assert isinstance(feature, route_guide_pb2.Feature)
    if not feature.name:
        print("received no feature")
    else:
        print(f"received feature name: {feature.name}")


def list_features(stub: route_guide_pb2_grpc.RouteGuideStub) -> None:
    rectangle = route_guide_pb2.Rectangle(
        lo=route_guide_pb2.Point(latitude=350000000, longitude=1390000000),
        hi=route_guide_pb2.Point(latitude=360000000, longitude=1400000000),
    )
    for feature in stub.ListFeatures(rectangle):
        assert isinstance(feature, route_guide_pb2.Feature)
        print(f"received feature name: {feature.name}")


def record_route(stub: route_guide_pb2_grpc.RouteGuideStub) -> None:
    features = load_db()
    points = (
        features[random.randint(0, len(features) - 1)].location.into_pb2_point()
        for _ in range(10)
    )
    summary = stub.RecordRoute(points)
    assert isinstance(summary, route_guide_pb2.RouteSummary)
    print(f"[route summary] point_count = {summary.point_count}")
    print(f"[route summary] feature_count = {summary.feature_count}")
    print(f"[route summary] distance = {summary.distance}")
    print(f"[route summary] elapsed_time = {summary.elapsed_time}")


def route_chat(stub: route_guide_pb2_grpc.RouteGuideStub) -> None:
    notes = [
        ((0, 0), "First message"),
        ((1, 0), "Second message"),
        ((2, 0), "Third message"),
        ((1, 0), "Fourth message"),
        ((0, 0), "Fifth message"),
    ]
    response = stub.RouteChat(
        route_guide_pb2.RouteNote(
            location=route_guide_pb2.Point(latitude=p[0], longitude=p[1]), message=m
        )
        for (p, m) in notes
    )
    for note in response:
        assert isinstance(note, route_guide_pb2.RouteNote)
        la = note.location.latitude
        lo = note.location.longitude
        msg = note.message
        print(f"message at ({la}, {lo}): {msg}")


def run() -> None:
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = route_guide_pb2_grpc.RouteGuideStub(channel)
        print("----- GetFeature -----")
        get_feature(stub)
        print("----- ListFeatures -----")
        list_features(stub)
        print("----- RecordRoute -----")
        record_route(stub)
        print("----- RouteChat -----")
        route_chat(stub)


if __name__ == "__main__":
    logging.basicConfig()
    run()

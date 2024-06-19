import logging

import grpc
from generated import route_guide_pb2, route_guide_pb2_grpc


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


def run() -> None:
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = route_guide_pb2_grpc.RouteGuideStub(channel)
        print("----- GetFeature -----")
        get_feature(stub)
        print("----- ListFeatures -----")
        list_features(stub)


if __name__ == "__main__":
    logging.basicConfig()
    run()

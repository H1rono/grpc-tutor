import logging

import grpc
from generated import route_guide_pb2, route_guide_pb2_grpc


def run() -> None:
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = route_guide_pb2_grpc.RouteGuideStub(channel)
        response = stub.GetFeature(
            route_guide_pb2.Point(latitude=356810420, longitude=1397672140)
        )
    print("Route guide client received: " + response.name)


if __name__ == "__main__":
    logging.basicConfig()
    run()

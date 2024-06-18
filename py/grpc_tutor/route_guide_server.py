from collections.abc import AsyncIterator, Awaitable, Iterator
from concurrent.futures import ThreadPoolExecutor

import grpc
from generated import route_guide_pb2, route_guide_pb2_grpc


class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
    def GetFeature(
        self,
        request: route_guide_pb2.Point,
        context: route_guide_pb2_grpc._ServicerContext,
    ) -> route_guide_pb2.Feature | Awaitable[route_guide_pb2.Feature]: ...  # TODO

    def ListFeatures(
        self,
        request: route_guide_pb2.Rectangle,
        context: route_guide_pb2_grpc._ServicerContext,
    ) -> (
        Iterator[route_guide_pb2.Feature] | AsyncIterator[route_guide_pb2.Feature]
    ): ...  # TODO

    def RecordRoute(
        self,
        request_iterator: route_guide_pb2_grpc._MaybeAsyncIterator[route_guide_pb2.Point],
        context: route_guide_pb2_grpc._ServicerContext,
    ) -> (
        route_guide_pb2.RouteSummary | Awaitable[route_guide_pb2.RouteSummary]
    ): ...  # TODO

    def RouteChat(
        self,
        request_iterator: route_guide_pb2_grpc._MaybeAsyncIterator[
            route_guide_pb2.RouteNote
        ],
        context: route_guide_pb2_grpc._ServicerContext,
    ) -> (
        Iterator[route_guide_pb2.RouteNote] | AsyncIterator[route_guide_pb2.RouteNote]
    ): ...  # TODO


def serve() -> None:
    with ThreadPoolExecutor(max_workers=10) as executor:
        server = grpc.server(executor)
        route_guide_pb2_grpc.add_RouteGuideServicer_to_server(
            RouteGuideServicer(), server
        )
        server.add_insecure_port("[::]:50051")
        server.start()
        server.wait_for_termination()

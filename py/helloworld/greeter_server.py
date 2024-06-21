# https://github.com/grpc/grpc/blob/b8a04ac/examples/python/helloworld/greeter_server.py
"""The Python implementation of the GRPC helloworld.Greeter server."""

import asyncio
import logging
from concurrent import futures

import grpc.aio

from . import helloworld_pb2, helloworld_pb2_grpc


class Greeter(helloworld_pb2_grpc.GreeterServicer):
    def SayHello(
        self, request: helloworld_pb2.HelloRequest, context: grpc.ServicerContext
    ) -> helloworld_pb2.HelloReply:
        return helloworld_pb2.HelloReply(message="Hello, %s!" % request.name)


class AsyncGreeter(helloworld_pb2_grpc.GreeterServicer):
    async def SayHello(
        self, request: helloworld_pb2.HelloRequest, context: grpc.ServicerContext
    ) -> helloworld_pb2.HelloReply:
        print(type(context))
        await asyncio.sleep(0.1)
        return helloworld_pb2.HelloReply(message=f"[async] Hello, {request.name}!")


def serve() -> None:
    port = "50051"
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    helloworld_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server.add_insecure_port("[::]:" + port)
    server.start()
    print("Server started, listening on " + port)
    server.wait_for_termination()


async def start_server(server: grpc.aio.Server) -> None:
    port = "50051"
    server = grpc.aio.server(futures.ThreadPoolExecutor(max_workers=10))
    helloworld_pb2_grpc.add_GreeterServicer_to_server(AsyncGreeter(), server)
    server.add_insecure_port(f"[::]:{port}")
    await server.start()
    print(f"Server started, listening on port {port}")
    await server.wait_for_termination()


def run_async_serve() -> None:
    loop = asyncio.get_event_loop()
    server = grpc.aio.server(futures.ThreadPoolExecutor(max_workers=10))
    try:
        loop.run_until_complete(start_server(server))
    finally:
        loop.run_until_complete(server.stop(1))
        loop.close()


if __name__ == "__main__":
    logging.basicConfig()
    serve()

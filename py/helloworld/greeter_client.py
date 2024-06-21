# https://github.com/grpc/grpc/blob/b8a04ac/examples/python/helloworld/greeter_client.py
"""The Python implementation of the GRPC helloworld.Greeter client."""

from __future__ import print_function

import asyncio
import logging

import grpc.aio

from . import helloworld_pb2, helloworld_pb2_grpc


def run() -> None:
    # NOTE(gRPC Python Team): .close() is possible on a channel and should be
    # used in circumstances in which the with statement does not fit the needs
    # of the code.
    print("Will try to greet world ...")
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = helloworld_pb2_grpc.GreeterStub(channel)
        response = stub.SayHello(helloworld_pb2.HelloRequest(name="you"))
    print("Greeter client received: " + response.message)


async def client_async() -> None:
    print("Will try to greet world ...")
    async with grpc.aio.insecure_channel("localhost:50051") as channel:
        stub = helloworld_pb2_grpc.GreeterStub(channel)
        response = await stub.SayHello(helloworld_pb2.HelloRequest(name="world"))
    print("Greeter client received: " + response.message)


def run_async() -> None:
    asyncio.run(client_async())


if __name__ == "__main__":
    logging.basicConfig()
    run()

#!/usr/bin/env bash

set -eux -o pipefail

for file in $(ls protos)
do
	: "process $file"
	rye run \
		python3 -m \
		grpc_tools.protoc \
		-Igenerated=./protos \
		--python_out=./py --grpc_python_out=./py \
		--mypy_out=./py --mypy_grpc_out=./py \
		--plugin=protoc-gen-mypy=./.venv/bin/protoc-gen-mypy \
		--plugin=ptotoc-gen-mypy_grpc=./.venv/bin/protoc-gen-mypy_grpc \
		"./protos/$file"
done

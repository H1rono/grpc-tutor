#!/usr/bin/env bash

set -eux -o pipefail

for file in $(ls protos)
do
	: "process $file"
	rye run \
		python3 -m \
		grpc_tools.protoc \
		-Igenerated=./protos \
		--python_out=./py \
		--pyi_out=./py \
		--grpc_python_out=./py \
		"./protos/$file"
done

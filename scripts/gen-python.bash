#!/usr/bin/env bash

set -eux -o pipefail

for file in $(ls protos)
do
	: "process $file"
	pkg_name=`cat "./protos/$file" | grep package | head -n 1 | sed -r 's/^package\s+(.*)\s*;$/\1/'`
	mkdir -p "./py/$pkg_name"
	touch "./py/$pkg_name/__init__.py"
	rye run \
		python3 -m \
		grpc_tools.protoc \
		-I${pkg_name}=./protos \
		--python_out=./py \
		--pyi_out=./py \
		--grpc_python_out=./py \
		"./protos/$file"
done

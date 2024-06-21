# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: route_guide/route_guide.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1droute_guide/route_guide.proto\x12\x0broute_guide\",\n\x05Point\x12\x10\n\x08latitude\x18\x01 \x01(\x05\x12\x11\n\tlongitude\x18\x02 \x01(\x05\"K\n\tRectangle\x12\x1e\n\x02lo\x18\x01 \x01(\x0b\x32\x12.route_guide.Point\x12\x1e\n\x02hi\x18\x02 \x01(\x0b\x32\x12.route_guide.Point\"=\n\x07\x46\x65\x61ture\x12\x0c\n\x04name\x18\x01 \x01(\t\x12$\n\x08location\x18\x02 \x01(\x0b\x32\x12.route_guide.Point\"B\n\tRouteNote\x12$\n\x08location\x18\x01 \x01(\x0b\x32\x12.route_guide.Point\x12\x0f\n\x07message\x18\x02 \x01(\t\"b\n\x0cRouteSummary\x12\x13\n\x0bpoint_count\x18\x01 \x01(\x05\x12\x15\n\rfeature_count\x18\x02 \x01(\x05\x12\x10\n\x08\x64istance\x18\x03 \x01(\x05\x12\x14\n\x0c\x65lapsed_time\x18\x04 \x01(\x05\x32\x8d\x02\n\nRouteGuide\x12\x38\n\nGetFeature\x12\x12.route_guide.Point\x1a\x14.route_guide.Feature\"\x00\x12@\n\x0cListFeatures\x12\x16.route_guide.Rectangle\x1a\x14.route_guide.Feature\"\x00\x30\x01\x12@\n\x0bRecordRoute\x12\x12.route_guide.Point\x1a\x19.route_guide.RouteSummary\"\x00(\x01\x12\x41\n\tRouteChat\x12\x16.route_guide.RouteNote\x1a\x16.route_guide.RouteNote\"\x00(\x01\x30\x01\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'route_guide.route_guide_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_POINT']._serialized_start=46
  _globals['_POINT']._serialized_end=90
  _globals['_RECTANGLE']._serialized_start=92
  _globals['_RECTANGLE']._serialized_end=167
  _globals['_FEATURE']._serialized_start=169
  _globals['_FEATURE']._serialized_end=230
  _globals['_ROUTENOTE']._serialized_start=232
  _globals['_ROUTENOTE']._serialized_end=298
  _globals['_ROUTESUMMARY']._serialized_start=300
  _globals['_ROUTESUMMARY']._serialized_end=398
  _globals['_ROUTEGUIDE']._serialized_start=401
  _globals['_ROUTEGUIDE']._serialized_end=670
# @@protoc_insertion_point(module_scope)
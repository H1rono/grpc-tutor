# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: addressbook/addressbook.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1d\x61\x64\x64ressbook/addressbook.proto\x12\x0b\x61\x64\x64ressbook\"\xdf\x02\n\x06Person\x12\x11\n\x04name\x18\x01 \x01(\tH\x00\x88\x01\x01\x12\x0f\n\x02id\x18\x02 \x01(\x05H\x01\x88\x01\x01\x12\x12\n\x05\x65mail\x18\x03 \x01(\tH\x02\x88\x01\x01\x12/\n\x06phones\x18\x04 \x03(\x0b\x32\x1f.addressbook.Person.PhoneNumber\x1ah\n\x0bPhoneNumber\x12\x13\n\x06number\x18\x01 \x01(\tH\x00\x88\x01\x01\x12\x30\n\x04type\x18\x02 \x01(\x0e\x32\x1d.addressbook.Person.PhoneTypeH\x01\x88\x01\x01\x42\t\n\x07_numberB\x07\n\x05_type\"h\n\tPhoneType\x12\x1a\n\x16PHONE_TYPE_UNSPECIFIED\x10\x00\x12\x15\n\x11PHONE_TYPE_MOBILE\x10\x01\x12\x13\n\x0fPHONE_TYPE_HOME\x10\x02\x12\x13\n\x0fPHONE_TYPE_WORK\x10\x03\x42\x07\n\x05_nameB\x05\n\x03_idB\x08\n\x06_email\"2\n\x0b\x41\x64\x64ressBook\x12#\n\x06people\x18\x01 \x03(\x0b\x32\x13.addressbook.Personb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'addressbook.addressbook_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_PERSON']._serialized_start=47
  _globals['_PERSON']._serialized_end=398
  _globals['_PERSON_PHONENUMBER']._serialized_start=162
  _globals['_PERSON_PHONENUMBER']._serialized_end=266
  _globals['_PERSON_PHONETYPE']._serialized_start=268
  _globals['_PERSON_PHONETYPE']._serialized_end=372
  _globals['_ADDRESSBOOK']._serialized_start=400
  _globals['_ADDRESSBOOK']._serialized_end=450
# @@protoc_insertion_point(module_scope)

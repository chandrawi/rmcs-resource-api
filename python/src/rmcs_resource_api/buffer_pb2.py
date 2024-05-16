# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rmcs_resource_api/buffer.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from rmcs_resource_api import common_pb2 as rmcs__resource__api_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1ermcs_resource_api/buffer.proto\x12\x06\x62uffer\x1a\x1ermcs_resource_api/common.proto\"\x9b\x01\n\x0c\x42ufferSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x11\n\ttimestamp\x18\x04 \x01(\x03\x12\x12\n\ndata_bytes\x18\x05 \x01(\x0c\x12#\n\tdata_type\x18\x06 \x03(\x0e\x32\x10.common.DataType\x12\x0e\n\x06status\x18\x07 \x01(\x05\"\x16\n\x08\x42ufferId\x12\n\n\x02id\x18\x01 \x01(\x05\"d\n\nBufferTime\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x13\n\x06status\x18\x04 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"z\n\x0e\x42ufferSelector\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12\x13\n\x06status\x18\x03 \x01(\x05H\x02\x88\x01\x01\x42\x0c\n\n_device_idB\x0b\n\t_model_idB\t\n\x07_status\"\x8b\x01\n\x0f\x42uffersSelector\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12\x13\n\x06status\x18\x03 \x01(\x05H\x02\x88\x01\x01\x12\x0e\n\x06number\x18\x04 \x01(\rB\x0c\n\n_device_idB\x0b\n\t_model_idB\t\n\x07_status\"\x87\x01\n\x0c\x42ufferUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x17\n\ndata_bytes\x18\x02 \x01(\x0cH\x00\x88\x01\x01\x12#\n\tdata_type\x18\x03 \x03(\x0e\x32\x10.common.DataType\x12\x13\n\x06status\x18\x04 \x01(\x05H\x01\x88\x01\x01\x42\r\n\x0b_data_bytesB\t\n\x07_status\":\n\x12\x42ufferReadResponse\x12$\n\x06result\x18\x01 \x01(\x0b\x32\x14.buffer.BufferSchema\";\n\x12\x42ufferListResponse\x12%\n\x07results\x18\x01 \x03(\x0b\x32\x14.buffer.BufferSchema\"\"\n\x14\x42ufferCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x16\n\x14\x42ufferChangeResponse2\xf3\x04\n\rBufferService\x12:\n\nReadBuffer\x12\x10.buffer.BufferId\x1a\x1a.buffer.BufferReadResponse\x12\x42\n\x10ReadBufferByTime\x12\x12.buffer.BufferTime\x1a\x1a.buffer.BufferReadResponse\x12\x45\n\x0fReadBufferFirst\x12\x16.buffer.BufferSelector\x1a\x1a.buffer.BufferReadResponse\x12\x44\n\x0eReadBufferLast\x12\x16.buffer.BufferSelector\x1a\x1a.buffer.BufferReadResponse\x12\x46\n\x0fListBufferFirst\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12\x45\n\x0eListBufferLast\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12\x42\n\x0c\x43reateBuffer\x12\x14.buffer.BufferSchema\x1a\x1c.buffer.BufferCreateResponse\x12\x42\n\x0cUpdateBuffer\x12\x14.buffer.BufferUpdate\x1a\x1c.buffer.BufferChangeResponse\x12>\n\x0c\x44\x65leteBuffer\x12\x10.buffer.BufferId\x1a\x1c.buffer.BufferChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.buffer_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_BUFFERSCHEMA']._serialized_start=75
  _globals['_BUFFERSCHEMA']._serialized_end=230
  _globals['_BUFFERID']._serialized_start=232
  _globals['_BUFFERID']._serialized_end=254
  _globals['_BUFFERTIME']._serialized_start=256
  _globals['_BUFFERTIME']._serialized_end=356
  _globals['_BUFFERSELECTOR']._serialized_start=358
  _globals['_BUFFERSELECTOR']._serialized_end=480
  _globals['_BUFFERSSELECTOR']._serialized_start=483
  _globals['_BUFFERSSELECTOR']._serialized_end=622
  _globals['_BUFFERUPDATE']._serialized_start=625
  _globals['_BUFFERUPDATE']._serialized_end=760
  _globals['_BUFFERREADRESPONSE']._serialized_start=762
  _globals['_BUFFERREADRESPONSE']._serialized_end=820
  _globals['_BUFFERLISTRESPONSE']._serialized_start=822
  _globals['_BUFFERLISTRESPONSE']._serialized_end=881
  _globals['_BUFFERCREATERESPONSE']._serialized_start=883
  _globals['_BUFFERCREATERESPONSE']._serialized_end=917
  _globals['_BUFFERCHANGERESPONSE']._serialized_start=919
  _globals['_BUFFERCHANGERESPONSE']._serialized_end=941
  _globals['_BUFFERSERVICE']._serialized_start=944
  _globals['_BUFFERSERVICE']._serialized_end=1571
# @@protoc_insertion_point(module_scope)

# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: buffer.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


import common_pb2 as common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0c\x62uffer.proto\x12\x06\x62uffer\x1a\x0c\x63ommon.proto\"\xc0\x01\n\x0c\x42ufferSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x11\n\ttimestamp\x18\x04 \x01(\x03\x12\r\n\x05index\x18\x05 \x01(\x05\x12\x12\n\ndata_bytes\x18\x06 \x01(\x0c\x12#\n\tdata_type\x18\x07 \x03(\x0e\x32\x10.common.DataType\x12$\n\x06status\x18\x08 \x01(\x0e\x32\x14.buffer.BufferStatus\"\x16\n\x08\x42ufferId\x12\n\n\x02id\x18\x01 \x01(\x05\"\x90\x01\n\x0e\x42ufferSelector\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12)\n\x06status\x18\x03 \x01(\x0e\x32\x14.buffer.BufferStatusH\x02\x88\x01\x01\x42\x0c\n\n_device_idB\x0b\n\t_model_idB\t\n\x07_status\"\xa1\x01\n\x0f\x42uffersSelector\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12)\n\x06status\x18\x03 \x01(\x0e\x32\x14.buffer.BufferStatusH\x02\x88\x01\x01\x12\x0e\n\x06number\x18\x04 \x01(\rB\x0c\n\n_device_idB\x0b\n\t_model_idB\t\n\x07_status\"\x9d\x01\n\x0c\x42ufferUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x17\n\ndata_bytes\x18\x06 \x01(\x0cH\x00\x88\x01\x01\x12#\n\tdata_type\x18\x07 \x03(\x0e\x32\x10.common.DataType\x12)\n\x06status\x18\x08 \x01(\x0e\x32\x14.buffer.BufferStatusH\x01\x88\x01\x01\x42\r\n\x0b_data_bytesB\t\n\x07_status\":\n\x12\x42ufferReadResponse\x12$\n\x06result\x18\x01 \x01(\x0b\x32\x14.buffer.BufferSchema\";\n\x12\x42ufferListResponse\x12%\n\x07results\x18\x01 \x03(\x0b\x32\x14.buffer.BufferSchema\"\"\n\x14\x42ufferCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x16\n\x14\x42ufferChangeResponse*\x9f\x01\n\x0c\x42ufferStatus\x12\x0b\n\x07\x44\x45\x46\x41ULT\x10\x00\x12\t\n\x05\x45RROR\x10\x01\x12\x0b\n\x07\x43ONVERT\x10\x02\x12\x13\n\x0f\x41NALYZE_GATEWAY\x10\x03\x12\x12\n\x0e\x41NALYZE_SERVER\x10\x04\x12\x14\n\x10TRANSFER_GATEWAY\x10\x05\x12\x13\n\x0fTRANSFER_SERVER\x10\x06\x12\n\n\x06\x42\x41\x43KUP\x10\x07\x12\n\n\x06\x44\x45LETE\x10\x08\x32\xaf\x04\n\rBufferService\x12:\n\nReadBuffer\x12\x10.buffer.BufferId\x1a\x1a.buffer.BufferReadResponse\x12\x45\n\x0fReadBufferFirst\x12\x16.buffer.BufferSelector\x1a\x1a.buffer.BufferReadResponse\x12\x44\n\x0eReadBufferLast\x12\x16.buffer.BufferSelector\x1a\x1a.buffer.BufferReadResponse\x12\x46\n\x0fListBufferFirst\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12\x45\n\x0eListBufferLast\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12\x42\n\x0c\x43reateBuffer\x12\x14.buffer.BufferSchema\x1a\x1c.buffer.BufferCreateResponse\x12\x42\n\x0cUpdateBuffer\x12\x14.buffer.BufferUpdate\x1a\x1c.buffer.BufferChangeResponse\x12>\n\x0c\x44\x65leteBuffer\x12\x10.buffer.BufferId\x1a\x1c.buffer.BufferChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'buffer_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_BUFFERSTATUS']._serialized_start=910
  _globals['_BUFFERSTATUS']._serialized_end=1069
  _globals['_BUFFERSCHEMA']._serialized_start=39
  _globals['_BUFFERSCHEMA']._serialized_end=231
  _globals['_BUFFERID']._serialized_start=233
  _globals['_BUFFERID']._serialized_end=255
  _globals['_BUFFERSELECTOR']._serialized_start=258
  _globals['_BUFFERSELECTOR']._serialized_end=402
  _globals['_BUFFERSSELECTOR']._serialized_start=405
  _globals['_BUFFERSSELECTOR']._serialized_end=566
  _globals['_BUFFERUPDATE']._serialized_start=569
  _globals['_BUFFERUPDATE']._serialized_end=726
  _globals['_BUFFERREADRESPONSE']._serialized_start=728
  _globals['_BUFFERREADRESPONSE']._serialized_end=786
  _globals['_BUFFERLISTRESPONSE']._serialized_start=788
  _globals['_BUFFERLISTRESPONSE']._serialized_end=847
  _globals['_BUFFERCREATERESPONSE']._serialized_start=849
  _globals['_BUFFERCREATERESPONSE']._serialized_end=883
  _globals['_BUFFERCHANGERESPONSE']._serialized_start=885
  _globals['_BUFFERCHANGERESPONSE']._serialized_end=907
  _globals['_BUFFERSERVICE']._serialized_start=1072
  _globals['_BUFFERSERVICE']._serialized_end=1631
# @@protoc_insertion_point(module_scope)

# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: rmcs_resource_api/buffer.proto
# Protobuf Python Version: 5.27.2
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    2,
    '',
    'rmcs_resource_api/buffer.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from rmcs_resource_api import common_pb2 as rmcs__resource__api_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1ermcs_resource_api/buffer.proto\x12\x06\x62uffer\x1a\x1ermcs_resource_api/common.proto\"\x9b\x01\n\x0c\x42ufferSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x11\n\ttimestamp\x18\x04 \x01(\x03\x12\x12\n\ndata_bytes\x18\x05 \x01(\x0c\x12#\n\tdata_type\x18\x06 \x03(\x0e\x32\x10.common.DataType\x12\x0e\n\x06status\x18\x07 \x01(\x05\"\x16\n\x08\x42ufferId\x12\n\n\x02id\x18\x01 \x01(\x05\"d\n\nBufferTime\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x13\n\x06status\x18\x04 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"n\n\x0b\x42ufferRange\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x03 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x04 \x01(\x03\x12\x13\n\x06status\x18\x05 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"v\n\x0c\x42ufferNumber\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x0e\n\x06number\x18\x04 \x01(\r\x12\x13\n\x06status\x18\x05 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"z\n\x0e\x42ufferSelector\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12\x13\n\x06status\x18\x03 \x01(\x05H\x02\x88\x01\x01\x42\x0c\n\n_device_idB\x0b\n\t_model_idB\t\n\x07_status\"\x9b\x01\n\x0f\x42uffersSelector\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12\x13\n\x06status\x18\x03 \x01(\x05H\x02\x88\x01\x01\x12\x0e\n\x06number\x18\x04 \x01(\r\x12\x0e\n\x06offset\x18\x05 \x01(\rB\x0c\n\n_device_idB\x0b\n\t_model_idB\t\n\x07_status\"i\n\rBufferIdsTime\x12\x12\n\ndevice_ids\x18\x01 \x03(\x0c\x12\x11\n\tmodel_ids\x18\x02 \x03(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x13\n\x06status\x18\x04 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"s\n\x0e\x42ufferIdsRange\x12\x12\n\ndevice_ids\x18\x01 \x03(\x0c\x12\x11\n\tmodel_ids\x18\x02 \x03(\x0c\x12\r\n\x05\x62\x65gin\x18\x03 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x04 \x01(\x03\x12\x13\n\x06status\x18\x05 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"{\n\x0f\x42ufferIdsNumber\x12\x12\n\ndevice_ids\x18\x01 \x03(\x0c\x12\x11\n\tmodel_ids\x18\x02 \x03(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x0e\n\x06number\x18\x04 \x01(\r\x12\x13\n\x06status\x18\x05 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"Z\n\x11\x42ufferIdsSelector\x12\x12\n\ndevice_ids\x18\x01 \x03(\x0c\x12\x11\n\tmodel_ids\x18\x02 \x03(\x0c\x12\x13\n\x06status\x18\x03 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"{\n\x12\x42uffersIdsSelector\x12\x12\n\ndevice_ids\x18\x01 \x03(\x0c\x12\x11\n\tmodel_ids\x18\x02 \x03(\x0c\x12\x13\n\x06status\x18\x03 \x01(\x05H\x00\x88\x01\x01\x12\x0e\n\x06number\x18\x04 \x01(\r\x12\x0e\n\x06offset\x18\x05 \x01(\rB\t\n\x07_status\"R\n\rBufferSetTime\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\x12\x13\n\x06status\x18\x03 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"\\\n\x0e\x42ufferSetRange\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x02 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x03 \x01(\x03\x12\x13\n\x06status\x18\x04 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"d\n\x0f\x42ufferSetNumber\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\x12\x0e\n\x06number\x18\x03 \x01(\r\x12\x13\n\x06status\x18\x04 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"C\n\x11\x42ufferSetSelector\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x13\n\x06status\x18\x02 \x01(\x05H\x00\x88\x01\x01\x42\t\n\x07_status\"d\n\x12\x42uffersSetSelector\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x13\n\x06status\x18\x02 \x01(\x05H\x00\x88\x01\x01\x12\x0e\n\x06number\x18\x03 \x01(\r\x12\x0e\n\x06offset\x18\x04 \x01(\rB\t\n\x07_status\"\x87\x01\n\x0c\x42ufferUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x17\n\ndata_bytes\x18\x02 \x01(\x0cH\x00\x88\x01\x01\x12#\n\tdata_type\x18\x03 \x03(\x0e\x32\x10.common.DataType\x12\x13\n\x06status\x18\x04 \x01(\x05H\x01\x88\x01\x01\x42\r\n\x0b_data_bytesB\t\n\x07_status\":\n\x12\x42ufferReadResponse\x12$\n\x06result\x18\x01 \x01(\x0b\x32\x14.buffer.BufferSchema\";\n\x12\x42ufferListResponse\x12%\n\x07results\x18\x01 \x03(\x0b\x32\x14.buffer.BufferSchema\"\"\n\x14\x42ufferCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x16\n\x14\x42ufferChangeResponse\"*\n\x15TimestampReadResponse\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\"+\n\x15TimestampListResponse\x12\x12\n\ntimestamps\x18\x01 \x03(\x03\"$\n\x13\x42ufferCountResponse\x12\r\n\x05\x63ount\x18\x01 \x01(\r2\x83\x1b\n\rBufferService\x12:\n\nReadBuffer\x12\x10.buffer.BufferId\x1a\x1a.buffer.BufferReadResponse\x12\x42\n\x10ReadBufferByTime\x12\x12.buffer.BufferTime\x1a\x1a.buffer.BufferReadResponse\x12\x46\n\x14ListBufferByLastTime\x12\x12.buffer.BufferTime\x1a\x1a.buffer.BufferListResponse\x12H\n\x15ListBufferByRangeTime\x12\x13.buffer.BufferRange\x1a\x1a.buffer.BufferListResponse\x12L\n\x18ListBufferByNumberBefore\x12\x14.buffer.BufferNumber\x1a\x1a.buffer.BufferListResponse\x12K\n\x17ListBufferByNumberAfter\x12\x14.buffer.BufferNumber\x1a\x1a.buffer.BufferListResponse\x12\x45\n\x0fReadBufferFirst\x12\x16.buffer.BufferSelector\x1a\x1a.buffer.BufferReadResponse\x12\x44\n\x0eReadBufferLast\x12\x16.buffer.BufferSelector\x1a\x1a.buffer.BufferReadResponse\x12\x46\n\x0fListBufferFirst\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12L\n\x15ListBufferFirstOffset\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12\x45\n\x0eListBufferLast\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12K\n\x14ListBufferLastOffset\x12\x17.buffer.BuffersSelector\x1a\x1a.buffer.BufferListResponse\x12H\n\x13ListBufferByIdsTime\x12\x15.buffer.BufferIdsTime\x1a\x1a.buffer.BufferListResponse\x12L\n\x17ListBufferByIdsLastTime\x12\x15.buffer.BufferIdsTime\x1a\x1a.buffer.BufferListResponse\x12N\n\x18ListBufferByIdsRangeTime\x12\x16.buffer.BufferIdsRange\x1a\x1a.buffer.BufferListResponse\x12R\n\x1bListBufferByIdsNumberBefore\x12\x17.buffer.BufferIdsNumber\x1a\x1a.buffer.BufferListResponse\x12Q\n\x1aListBufferByIdsNumberAfter\x12\x17.buffer.BufferIdsNumber\x1a\x1a.buffer.BufferListResponse\x12N\n\x14ListBufferFirstByIds\x12\x1a.buffer.BuffersIdsSelector\x1a\x1a.buffer.BufferListResponse\x12T\n\x1aListBufferFirstOffsetByIds\x12\x1a.buffer.BuffersIdsSelector\x1a\x1a.buffer.BufferListResponse\x12M\n\x13ListBufferLastByIds\x12\x1a.buffer.BuffersIdsSelector\x1a\x1a.buffer.BufferListResponse\x12S\n\x19ListBufferLastOffsetByIds\x12\x1a.buffer.BuffersIdsSelector\x1a\x1a.buffer.BufferListResponse\x12H\n\x13ListBufferBySetTime\x12\x15.buffer.BufferSetTime\x1a\x1a.buffer.BufferListResponse\x12L\n\x17ListBufferBySetLastTime\x12\x15.buffer.BufferSetTime\x1a\x1a.buffer.BufferListResponse\x12N\n\x18ListBufferBySetRangeTime\x12\x16.buffer.BufferSetRange\x1a\x1a.buffer.BufferListResponse\x12R\n\x1bListBufferBySetNumberBefore\x12\x17.buffer.BufferSetNumber\x1a\x1a.buffer.BufferListResponse\x12Q\n\x1aListBufferBySetNumberAfter\x12\x17.buffer.BufferSetNumber\x1a\x1a.buffer.BufferListResponse\x12N\n\x14ListBufferFirstBySet\x12\x1a.buffer.BuffersSetSelector\x1a\x1a.buffer.BufferListResponse\x12T\n\x1aListBufferFirstOffsetBySet\x12\x1a.buffer.BuffersSetSelector\x1a\x1a.buffer.BufferListResponse\x12M\n\x13ListBufferLastBySet\x12\x1a.buffer.BuffersSetSelector\x1a\x1a.buffer.BufferListResponse\x12S\n\x19ListBufferLastOffsetBySet\x12\x1a.buffer.BuffersSetSelector\x1a\x1a.buffer.BufferListResponse\x12\x42\n\x0c\x43reateBuffer\x12\x14.buffer.BufferSchema\x1a\x1c.buffer.BufferCreateResponse\x12\x42\n\x0cUpdateBuffer\x12\x14.buffer.BufferUpdate\x1a\x1c.buffer.BufferChangeResponse\x12>\n\x0c\x44\x65leteBuffer\x12\x10.buffer.BufferId\x1a\x1c.buffer.BufferChangeResponse\x12Q\n\x18ReadBufferTimestampFirst\x12\x16.buffer.BufferSelector\x1a\x1d.buffer.TimestampReadResponse\x12P\n\x17ReadBufferTimestampLast\x12\x16.buffer.BufferSelector\x1a\x1d.buffer.TimestampReadResponse\x12R\n\x18ListBufferTimestampFirst\x12\x17.buffer.BuffersSelector\x1a\x1d.buffer.TimestampListResponse\x12Q\n\x17ListBufferTimestampLast\x12\x17.buffer.BuffersSelector\x1a\x1d.buffer.TimestampListResponse\x12Z\n\x1dListBufferTimestampFirstByIds\x12\x1a.buffer.BuffersIdsSelector\x1a\x1d.buffer.TimestampListResponse\x12Y\n\x1cListBufferTimestampLastByIds\x12\x1a.buffer.BuffersIdsSelector\x1a\x1d.buffer.TimestampListResponse\x12Z\n\x1dListBufferTimestampFirstBySet\x12\x1a.buffer.BuffersSetSelector\x1a\x1d.buffer.TimestampListResponse\x12Y\n\x1cListBufferTimestampLastBySet\x12\x1a.buffer.BuffersSetSelector\x1a\x1d.buffer.TimestampListResponse\x12\x42\n\x0b\x43ountBuffer\x12\x16.buffer.BufferSelector\x1a\x1b.buffer.BufferCountResponse\x12J\n\x10\x43ountBufferByIds\x12\x19.buffer.BufferIdsSelector\x1a\x1b.buffer.BufferCountResponse\x12J\n\x10\x43ountBufferBySet\x12\x19.buffer.BufferSetSelector\x1a\x1b.buffer.BufferCountResponseb\x06proto3')

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
  _globals['_BUFFERRANGE']._serialized_start=358
  _globals['_BUFFERRANGE']._serialized_end=468
  _globals['_BUFFERNUMBER']._serialized_start=470
  _globals['_BUFFERNUMBER']._serialized_end=588
  _globals['_BUFFERSELECTOR']._serialized_start=590
  _globals['_BUFFERSELECTOR']._serialized_end=712
  _globals['_BUFFERSSELECTOR']._serialized_start=715
  _globals['_BUFFERSSELECTOR']._serialized_end=870
  _globals['_BUFFERIDSTIME']._serialized_start=872
  _globals['_BUFFERIDSTIME']._serialized_end=977
  _globals['_BUFFERIDSRANGE']._serialized_start=979
  _globals['_BUFFERIDSRANGE']._serialized_end=1094
  _globals['_BUFFERIDSNUMBER']._serialized_start=1096
  _globals['_BUFFERIDSNUMBER']._serialized_end=1219
  _globals['_BUFFERIDSSELECTOR']._serialized_start=1221
  _globals['_BUFFERIDSSELECTOR']._serialized_end=1311
  _globals['_BUFFERSIDSSELECTOR']._serialized_start=1313
  _globals['_BUFFERSIDSSELECTOR']._serialized_end=1436
  _globals['_BUFFERSETTIME']._serialized_start=1438
  _globals['_BUFFERSETTIME']._serialized_end=1520
  _globals['_BUFFERSETRANGE']._serialized_start=1522
  _globals['_BUFFERSETRANGE']._serialized_end=1614
  _globals['_BUFFERSETNUMBER']._serialized_start=1616
  _globals['_BUFFERSETNUMBER']._serialized_end=1716
  _globals['_BUFFERSETSELECTOR']._serialized_start=1718
  _globals['_BUFFERSETSELECTOR']._serialized_end=1785
  _globals['_BUFFERSSETSELECTOR']._serialized_start=1787
  _globals['_BUFFERSSETSELECTOR']._serialized_end=1887
  _globals['_BUFFERUPDATE']._serialized_start=1890
  _globals['_BUFFERUPDATE']._serialized_end=2025
  _globals['_BUFFERREADRESPONSE']._serialized_start=2027
  _globals['_BUFFERREADRESPONSE']._serialized_end=2085
  _globals['_BUFFERLISTRESPONSE']._serialized_start=2087
  _globals['_BUFFERLISTRESPONSE']._serialized_end=2146
  _globals['_BUFFERCREATERESPONSE']._serialized_start=2148
  _globals['_BUFFERCREATERESPONSE']._serialized_end=2182
  _globals['_BUFFERCHANGERESPONSE']._serialized_start=2184
  _globals['_BUFFERCHANGERESPONSE']._serialized_end=2206
  _globals['_TIMESTAMPREADRESPONSE']._serialized_start=2208
  _globals['_TIMESTAMPREADRESPONSE']._serialized_end=2250
  _globals['_TIMESTAMPLISTRESPONSE']._serialized_start=2252
  _globals['_TIMESTAMPLISTRESPONSE']._serialized_end=2295
  _globals['_BUFFERCOUNTRESPONSE']._serialized_start=2297
  _globals['_BUFFERCOUNTRESPONSE']._serialized_end=2333
  _globals['_BUFFERSERVICE']._serialized_start=2336
  _globals['_BUFFERSERVICE']._serialized_end=5795
# @@protoc_insertion_point(module_scope)

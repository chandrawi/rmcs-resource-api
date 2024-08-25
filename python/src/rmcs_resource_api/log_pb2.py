# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: rmcs_resource_api/log.proto
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
    'rmcs_resource_api/log.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1brmcs_resource_api/log.proto\x12\x03log\"f\n\tLogSchema\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x0e\n\x06status\x18\x03 \x01(\x05\x12\x11\n\tlog_bytes\x18\x04 \x01(\x0c\x12\x10\n\x08log_type\x18\x05 \x01(\r\"-\n\x05LogId\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\"b\n\x07LogTime\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x16\n\tdevice_id\x18\x02 \x01(\x0cH\x00\x88\x01\x01\x12\x13\n\x06status\x18\x03 \x01(\x05H\x01\x88\x01\x01\x42\x0c\n\n_device_idB\t\n\x07_status\"l\n\x08LogRange\x12\r\n\x05\x62\x65gin\x18\x01 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x02 \x01(\x03\x12\x16\n\tdevice_id\x18\x03 \x01(\x0cH\x00\x88\x01\x01\x12\x13\n\x06status\x18\x04 \x01(\x05H\x01\x88\x01\x01\x42\x0c\n\n_device_idB\t\n\x07_status\"\x9b\x01\n\tLogUpdate\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x13\n\x06status\x18\x03 \x01(\x05H\x00\x88\x01\x01\x12\x16\n\tlog_bytes\x18\x04 \x01(\x0cH\x01\x88\x01\x01\x12\x15\n\x08log_type\x18\x05 \x01(\rH\x02\x88\x01\x01\x42\t\n\x07_statusB\x0c\n\n_log_bytesB\x0b\n\t_log_type\"1\n\x0fLogReadResponse\x12\x1e\n\x06result\x18\x01 \x01(\x0b\x32\x0e.log.LogSchema\"2\n\x0fLogListResponse\x12\x1f\n\x07results\x18\x01 \x03(\x0b\x32\x0e.log.LogSchema\"\x13\n\x11LogChangeResponse2\xfd\x02\n\nLogService\x12+\n\x07ReadLog\x12\n.log.LogId\x1a\x14.log.LogReadResponse\x12\x33\n\rListLogByTime\x12\x0c.log.LogTime\x1a\x14.log.LogListResponse\x12\x37\n\x11ListLogByLastTime\x12\x0c.log.LogTime\x1a\x14.log.LogListResponse\x12\x39\n\x12ListLogByRangeTime\x12\r.log.LogRange\x1a\x14.log.LogListResponse\x12\x33\n\tCreateLog\x12\x0e.log.LogSchema\x1a\x16.log.LogChangeResponse\x12\x33\n\tUpdateLog\x12\x0e.log.LogUpdate\x1a\x16.log.LogChangeResponse\x12/\n\tDeleteLog\x12\n.log.LogId\x1a\x16.log.LogChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.log_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_LOGSCHEMA']._serialized_start=36
  _globals['_LOGSCHEMA']._serialized_end=138
  _globals['_LOGID']._serialized_start=140
  _globals['_LOGID']._serialized_end=185
  _globals['_LOGTIME']._serialized_start=187
  _globals['_LOGTIME']._serialized_end=285
  _globals['_LOGRANGE']._serialized_start=287
  _globals['_LOGRANGE']._serialized_end=395
  _globals['_LOGUPDATE']._serialized_start=398
  _globals['_LOGUPDATE']._serialized_end=553
  _globals['_LOGREADRESPONSE']._serialized_start=555
  _globals['_LOGREADRESPONSE']._serialized_end=604
  _globals['_LOGLISTRESPONSE']._serialized_start=606
  _globals['_LOGLISTRESPONSE']._serialized_end=656
  _globals['_LOGCHANGERESPONSE']._serialized_start=658
  _globals['_LOGCHANGERESPONSE']._serialized_end=677
  _globals['_LOGSERVICE']._serialized_start=680
  _globals['_LOGSERVICE']._serialized_end=1061
# @@protoc_insertion_point(module_scope)

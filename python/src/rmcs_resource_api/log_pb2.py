# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rmcs_resource_api/log.proto
# Protobuf Python Version: 4.25.0
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from rmcs_resource_api import common_pb2 as rmcs__resource__api_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1brmcs_resource_api/log.proto\x12\x03log\x1a\x1ermcs_resource_api/common.proto\"\x8a\x01\n\tLogSchema\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x1e\n\x06status\x18\x03 \x01(\x0e\x32\x0e.log.LogStatus\x12\x11\n\tlog_bytes\x18\x04 \x01(\x0c\x12$\n\x08log_type\x18\x05 \x01(\x0e\x32\x12.common.ConfigType\"-\n\x05LogId\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\"r\n\x07LogTime\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x16\n\tdevice_id\x18\x02 \x01(\x0cH\x00\x88\x01\x01\x12#\n\x06status\x18\x03 \x01(\x0e\x32\x0e.log.LogStatusH\x01\x88\x01\x01\x42\x0c\n\n_device_idB\t\n\x07_status\"|\n\x08LogRange\x12\r\n\x05\x62\x65gin\x18\x01 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x02 \x01(\x03\x12\x16\n\tdevice_id\x18\x03 \x01(\x0cH\x00\x88\x01\x01\x12#\n\x06status\x18\x04 \x01(\x0e\x32\x0e.log.LogStatusH\x01\x88\x01\x01\x42\x0c\n\n_device_idB\t\n\x07_status\"\xbf\x01\n\tLogUpdate\x12\x11\n\ttimestamp\x18\x01 \x01(\x03\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12#\n\x06status\x18\x03 \x01(\x0e\x32\x0e.log.LogStatusH\x00\x88\x01\x01\x12\x16\n\tlog_bytes\x18\x04 \x01(\x0cH\x01\x88\x01\x01\x12)\n\x08log_type\x18\x05 \x01(\x0e\x32\x12.common.ConfigTypeH\x02\x88\x01\x01\x42\t\n\x07_statusB\x0c\n\n_log_bytesB\x0b\n\t_log_type\"1\n\x0fLogReadResponse\x12\x1e\n\x06result\x18\x01 \x01(\x0b\x32\x0e.log.LogSchema\"2\n\x0fLogListResponse\x12\x1f\n\x07results\x18\x01 \x03(\x0b\x32\x0e.log.LogSchema\"\x13\n\x11LogChangeResponse*\xbb\x02\n\tLogStatus\x12\x0b\n\x07\x44\x45\x46\x41ULT\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\r\n\tERROR_RAW\x10\x02\x12\x11\n\rERROR_MISSING\x10\x03\x12\x14\n\x10\x45RROR_CONVERSION\x10\x04\x12\x11\n\rERROR_ANALYZE\x10\x05\x12\x11\n\rERROR_NETWORK\x10\x06\x12\r\n\tFAIL_READ\x10\x07\x12\x0f\n\x0b\x46\x41IL_CREATE\x10\x08\x12\x0f\n\x0b\x46\x41IL_UPDATE\x10\t\x12\x0f\n\x0b\x46\x41IL_DELETE\x10\n\x12\x11\n\rINVALID_TOKEN\x10\x0b\x12\x13\n\x0fINVALID_REQUEST\x10\x0c\x12\r\n\tNOT_FOUND\x10\r\x12\x16\n\x12METHOD_NOT_ALLOWED\x10\x0e\x12\x11\n\rUNKNOWN_ERROR\x10\x0f\x12\x12\n\x0eUNKNOWN_STATUS\x10\x10\x32\xfd\x02\n\nLogService\x12+\n\x07ReadLog\x12\n.log.LogId\x1a\x14.log.LogReadResponse\x12\x33\n\rListLogByTime\x12\x0c.log.LogTime\x1a\x14.log.LogListResponse\x12\x37\n\x11ListLogByLastTime\x12\x0c.log.LogTime\x1a\x14.log.LogListResponse\x12\x39\n\x12ListLogByRangeTime\x12\r.log.LogRange\x1a\x14.log.LogListResponse\x12\x33\n\tCreateLog\x12\x0e.log.LogSchema\x1a\x16.log.LogChangeResponse\x12\x33\n\tUpdateLog\x12\x0e.log.LogUpdate\x1a\x16.log.LogChangeResponse\x12/\n\tDeleteLog\x12\n.log.LogId\x1a\x16.log.LogChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.log_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  DESCRIPTOR._options = None
  _globals['_LOGSTATUS']._serialized_start=817
  _globals['_LOGSTATUS']._serialized_end=1132
  _globals['_LOGSCHEMA']._serialized_start=69
  _globals['_LOGSCHEMA']._serialized_end=207
  _globals['_LOGID']._serialized_start=209
  _globals['_LOGID']._serialized_end=254
  _globals['_LOGTIME']._serialized_start=256
  _globals['_LOGTIME']._serialized_end=370
  _globals['_LOGRANGE']._serialized_start=372
  _globals['_LOGRANGE']._serialized_end=496
  _globals['_LOGUPDATE']._serialized_start=499
  _globals['_LOGUPDATE']._serialized_end=690
  _globals['_LOGREADRESPONSE']._serialized_start=692
  _globals['_LOGREADRESPONSE']._serialized_end=741
  _globals['_LOGLISTRESPONSE']._serialized_start=743
  _globals['_LOGLISTRESPONSE']._serialized_end=793
  _globals['_LOGCHANGERESPONSE']._serialized_start=795
  _globals['_LOGCHANGERESPONSE']._serialized_end=814
  _globals['_LOGSERVICE']._serialized_start=1135
  _globals['_LOGSERVICE']._serialized_end=1516
# @@protoc_insertion_point(module_scope)
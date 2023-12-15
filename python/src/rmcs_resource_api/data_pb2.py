# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rmcs_resource_api/data.proto
# Protobuf Python Version: 4.25.0
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from rmcs_resource_api import common_pb2 as rmcs__resource__api_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1crmcs_resource_api/data.proto\x12\x04\x64\x61ta\x1a\x1ermcs_resource_api/common.proto\"}\n\nDataSchema\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x12\n\ndata_bytes\x18\x04 \x01(\x0c\x12#\n\tdata_type\x18\x05 \x03(\x0e\x32\x10.common.DataType\"@\n\x06\x44\x61taId\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"B\n\x08\x44\x61taTime\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"L\n\tDataRange\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x03 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x04 \x01(\x03\"T\n\nDataNumber\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x0e\n\x06number\x18\x04 \x01(\r\"\x15\n\x07ModelId\x12\n\n\x02id\x18\x01 \x01(\x0c\"8\n\tDataModel\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x1f\n\x05types\x18\x02 \x03(\x0e\x32\x10.common.DataType\"\x90\x01\n\x0f\x44\x61taSchemaModel\x12\x1e\n\x05model\x18\x01 \x01(\x0b\x32\x0f.data.DataModel\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x12\n\ndata_bytes\x18\x04 \x01(\x0c\x12#\n\tdata_type\x18\x05 \x03(\x0e\x32\x10.common.DataType\"S\n\x0b\x44\x61taIdModel\x12\x1e\n\x05model\x18\x01 \x01(\x0b\x32\x0f.data.DataModel\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"U\n\rDataTimeModel\x12\x1e\n\x05model\x18\x01 \x01(\x0b\x32\x0f.data.DataModel\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"_\n\x0e\x44\x61taRangeModel\x12\x1e\n\x05model\x18\x01 \x01(\x0b\x32\x0f.data.DataModel\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x03 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x04 \x01(\x03\"g\n\x0f\x44\x61taNumberModel\x12\x1e\n\x05model\x18\x01 \x01(\x0b\x32\x0f.data.DataModel\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x0e\n\x06number\x18\x04 \x01(\r\"4\n\x10\x44\x61taReadResponse\x12 \n\x06result\x18\x01 \x01(\x0b\x32\x10.data.DataSchema\"5\n\x10\x44\x61taListResponse\x12!\n\x07results\x18\x01 \x03(\x0b\x32\x10.data.DataSchema\"4\n\x11\x44\x61taModelResponse\x12\x1f\n\x06result\x18\x01 \x01(\x0b\x32\x0f.data.DataModel\"\x14\n\x12\x44\x61taChangeResponse2\xf7\x08\n\x0b\x44\x61taService\x12\x30\n\x08ReadData\x12\x0c.data.DataId\x1a\x16.data.DataReadResponse\x12\x38\n\x0eListDataByTime\x12\x0e.data.DataTime\x1a\x16.data.DataListResponse\x12<\n\x12ListDataByLastTime\x12\x0e.data.DataTime\x1a\x16.data.DataListResponse\x12>\n\x13ListDataByRangeTime\x12\x0f.data.DataRange\x1a\x16.data.DataListResponse\x12\x42\n\x16ListDataByNumberBefore\x12\x10.data.DataNumber\x1a\x16.data.DataListResponse\x12\x41\n\x15ListDataByNumberAfter\x12\x10.data.DataNumber\x1a\x16.data.DataListResponse\x12\x36\n\x0cGetDataModel\x12\r.data.ModelId\x1a\x17.data.DataModelResponse\x12>\n\x11ReadDataWithModel\x12\x11.data.DataIdModel\x1a\x16.data.DataReadResponse\x12\x46\n\x17ListDataWithModelByTime\x12\x13.data.DataTimeModel\x1a\x16.data.DataListResponse\x12J\n\x1bListDataWithModelByLastTime\x12\x13.data.DataTimeModel\x1a\x16.data.DataListResponse\x12L\n\x1cListDataWithModelByRangeTime\x12\x14.data.DataRangeModel\x1a\x16.data.DataListResponse\x12P\n\x1fListDataWithModelByNumberBefore\x12\x15.data.DataNumberModel\x1a\x16.data.DataListResponse\x12O\n\x1eListDataWithModelByNumberAfter\x12\x15.data.DataNumberModel\x1a\x16.data.DataListResponse\x12\x38\n\nCreateData\x12\x10.data.DataSchema\x1a\x18.data.DataChangeResponse\x12\x46\n\x13\x43reateDataWithModel\x12\x15.data.DataSchemaModel\x1a\x18.data.DataChangeResponse\x12\x34\n\nDeleteData\x12\x0c.data.DataId\x1a\x18.data.DataChangeResponse\x12\x42\n\x13\x44\x65leteDataWithModel\x12\x11.data.DataIdModel\x1a\x18.data.DataChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.data_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  DESCRIPTOR._options = None
  _globals['_DATASCHEMA']._serialized_start=70
  _globals['_DATASCHEMA']._serialized_end=195
  _globals['_DATAID']._serialized_start=197
  _globals['_DATAID']._serialized_end=261
  _globals['_DATATIME']._serialized_start=263
  _globals['_DATATIME']._serialized_end=329
  _globals['_DATARANGE']._serialized_start=331
  _globals['_DATARANGE']._serialized_end=407
  _globals['_DATANUMBER']._serialized_start=409
  _globals['_DATANUMBER']._serialized_end=493
  _globals['_MODELID']._serialized_start=495
  _globals['_MODELID']._serialized_end=516
  _globals['_DATAMODEL']._serialized_start=518
  _globals['_DATAMODEL']._serialized_end=574
  _globals['_DATASCHEMAMODEL']._serialized_start=577
  _globals['_DATASCHEMAMODEL']._serialized_end=721
  _globals['_DATAIDMODEL']._serialized_start=723
  _globals['_DATAIDMODEL']._serialized_end=806
  _globals['_DATATIMEMODEL']._serialized_start=808
  _globals['_DATATIMEMODEL']._serialized_end=893
  _globals['_DATARANGEMODEL']._serialized_start=895
  _globals['_DATARANGEMODEL']._serialized_end=990
  _globals['_DATANUMBERMODEL']._serialized_start=992
  _globals['_DATANUMBERMODEL']._serialized_end=1095
  _globals['_DATAREADRESPONSE']._serialized_start=1097
  _globals['_DATAREADRESPONSE']._serialized_end=1149
  _globals['_DATALISTRESPONSE']._serialized_start=1151
  _globals['_DATALISTRESPONSE']._serialized_end=1204
  _globals['_DATAMODELRESPONSE']._serialized_start=1206
  _globals['_DATAMODELRESPONSE']._serialized_end=1258
  _globals['_DATACHANGERESPONSE']._serialized_start=1260
  _globals['_DATACHANGERESPONSE']._serialized_end=1280
  _globals['_DATASERVICE']._serialized_start=1283
  _globals['_DATASERVICE']._serialized_end=2426
# @@protoc_insertion_point(module_scope)

# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rmcs_resource_api/data.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from rmcs_resource_api import common_pb2 as rmcs__resource__api_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1crmcs_resource_api/data.proto\x12\x04\x64\x61ta\x1a\x1ermcs_resource_api/common.proto\"}\n\nDataSchema\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x12\n\ndata_bytes\x18\x04 \x01(\x0c\x12#\n\tdata_type\x18\x05 \x03(\x0e\x32\x10.common.DataType\"@\n\x06\x44\x61taId\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"B\n\x08\x44\x61taTime\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"L\n\tDataRange\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x03 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x04 \x01(\x03\"T\n\nDataNumber\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\x12\x0e\n\x06number\x18\x04 \x01(\r\"k\n\rDataSetSchema\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\x12\x12\n\ndata_bytes\x18\x03 \x01(\x0c\x12#\n\tdata_type\x18\x04 \x03(\x0e\x32\x10.common.DataType\".\n\tDataSetId\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\"0\n\x0b\x44\x61taSetTime\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\":\n\x0c\x44\x61taSetRange\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x02 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x03 \x01(\x03\"B\n\rDataSetNumber\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\x12\x0e\n\x06number\x18\x03 \x01(\r\"4\n\x10\x44\x61taReadResponse\x12 \n\x06result\x18\x01 \x01(\x0b\x32\x10.data.DataSchema\"5\n\x10\x44\x61taListResponse\x12!\n\x07results\x18\x01 \x03(\x0b\x32\x10.data.DataSchema\"\x14\n\x12\x44\x61taChangeResponse\":\n\x13\x44\x61taSetReadResponse\x12#\n\x06result\x18\x01 \x01(\x0b\x32\x13.data.DataSetSchema\";\n\x13\x44\x61taSetListResponse\x12$\n\x07results\x18\x01 \x03(\x0b\x32\x13.data.DataSetSchema2\xf2\t\n\x0b\x44\x61taService\x12\x30\n\x08ReadData\x12\x0c.data.DataId\x1a\x16.data.DataReadResponse\x12\x38\n\x0eListDataByTime\x12\x0e.data.DataTime\x1a\x16.data.DataListResponse\x12<\n\x12ListDataByLastTime\x12\x0e.data.DataTime\x1a\x16.data.DataListResponse\x12>\n\x13ListDataByRangeTime\x12\x0f.data.DataRange\x1a\x16.data.DataListResponse\x12\x42\n\x16ListDataByNumberBefore\x12\x10.data.DataNumber\x1a\x16.data.DataListResponse\x12\x41\n\x15ListDataByNumberAfter\x12\x10.data.DataNumber\x1a\x16.data.DataListResponse\x12\x38\n\nCreateData\x12\x10.data.DataSchema\x1a\x18.data.DataChangeResponse\x12\x34\n\nDeleteData\x12\x0c.data.DataId\x1a\x18.data.DataChangeResponse\x12>\n\x11ListDataBySetTime\x12\x11.data.DataSetTime\x1a\x16.data.DataListResponse\x12\x42\n\x15ListDataBySetLastTime\x12\x11.data.DataSetTime\x1a\x16.data.DataListResponse\x12\x44\n\x16ListDataBySetRangeTime\x12\x12.data.DataSetRange\x1a\x16.data.DataListResponse\x12H\n\x19ListDataBySetNumberBefore\x12\x13.data.DataSetNumber\x1a\x16.data.DataListResponse\x12G\n\x18ListDataBySetNumberAfter\x12\x13.data.DataSetNumber\x1a\x16.data.DataListResponse\x12\x39\n\x0bReadDataSet\x12\x0f.data.DataSetId\x1a\x19.data.DataSetReadResponse\x12\x41\n\x11ListDataSetByTime\x12\x11.data.DataSetTime\x1a\x19.data.DataSetListResponse\x12\x45\n\x15ListDataSetByLastTime\x12\x11.data.DataSetTime\x1a\x19.data.DataSetListResponse\x12G\n\x16ListDataSetByRangeTime\x12\x12.data.DataSetRange\x1a\x19.data.DataSetListResponse\x12K\n\x19ListDataSetByNumberBefore\x12\x13.data.DataSetNumber\x1a\x19.data.DataSetListResponse\x12J\n\x18ListDataSetByNumberAfter\x12\x13.data.DataSetNumber\x1a\x19.data.DataSetListResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.data_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
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
  _globals['_DATASETSCHEMA']._serialized_start=495
  _globals['_DATASETSCHEMA']._serialized_end=602
  _globals['_DATASETID']._serialized_start=604
  _globals['_DATASETID']._serialized_end=650
  _globals['_DATASETTIME']._serialized_start=652
  _globals['_DATASETTIME']._serialized_end=700
  _globals['_DATASETRANGE']._serialized_start=702
  _globals['_DATASETRANGE']._serialized_end=760
  _globals['_DATASETNUMBER']._serialized_start=762
  _globals['_DATASETNUMBER']._serialized_end=828
  _globals['_DATAREADRESPONSE']._serialized_start=830
  _globals['_DATAREADRESPONSE']._serialized_end=882
  _globals['_DATALISTRESPONSE']._serialized_start=884
  _globals['_DATALISTRESPONSE']._serialized_end=937
  _globals['_DATACHANGERESPONSE']._serialized_start=939
  _globals['_DATACHANGERESPONSE']._serialized_end=959
  _globals['_DATASETREADRESPONSE']._serialized_start=961
  _globals['_DATASETREADRESPONSE']._serialized_end=1019
  _globals['_DATASETLISTRESPONSE']._serialized_start=1021
  _globals['_DATASETLISTRESPONSE']._serialized_end=1080
  _globals['_DATASERVICE']._serialized_start=1083
  _globals['_DATASERVICE']._serialized_end=2349
# @@protoc_insertion_point(module_scope)

# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rmcs_resource_api/model.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from rmcs_resource_api import common_pb2 as rmcs__resource__api_dot_common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1drmcs_resource_api/model.proto\x12\x05model\x1a\x1ermcs_resource_api/common.proto\"\xc0\x01\n\x0bModelSchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12&\n\x08indexing\x18\x02 \x01(\x0e\x32\x14.common.DataIndexing\x12\x10\n\x08\x63\x61tegory\x18\x03 \x01(\t\x12\x0c\n\x04name\x18\x04 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x05 \x01(\t\x12\x1f\n\x05types\x18\x06 \x03(\x0e\x32\x10.common.DataType\x12\'\n\x07\x63onfigs\x18\x07 \x03(\x0b\x32\x16.model.ConfigSchemaVec\"7\n\x0f\x43onfigSchemaVec\x12$\n\x07\x63onfigs\x18\x01 \x03(\x0b\x32\x13.model.ConfigSchema\"\x15\n\x07ModelId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x19\n\tModelName\x12\x0c\n\x04name\x18\x01 \x01(\t\"!\n\rModelCategory\x12\x10\n\x08\x63\x61tegory\x18\x01 \x01(\t\"3\n\x11ModelNameCategory\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x10\n\x08\x63\x61tegory\x18\x02 \x01(\t\"\xbd\x01\n\x0bModelUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12+\n\x08indexing\x18\x02 \x01(\x0e\x32\x14.common.DataIndexingH\x00\x88\x01\x01\x12\x15\n\x08\x63\x61tegory\x18\x03 \x01(\tH\x01\x88\x01\x01\x12\x11\n\x04name\x18\x04 \x01(\tH\x02\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x05 \x01(\tH\x03\x88\x01\x01\x42\x0b\n\t_indexingB\x0b\n\t_categoryB\x07\n\x05_nameB\x0e\n\x0c_description\"9\n\nModelTypes\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x1f\n\x05types\x18\x06 \x03(\x0e\x32\x10.common.DataType\"\x9a\x01\n\x0c\x43onfigSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\r\n\x05index\x18\x03 \x01(\x05\x12\x0c\n\x04name\x18\x04 \x01(\t\x12\x14\n\x0c\x63onfig_bytes\x18\x05 \x01(\x0c\x12\'\n\x0b\x63onfig_type\x18\x06 \x01(\x0e\x32\x12.common.ConfigType\x12\x10\n\x08\x63\x61tegory\x18\x07 \x01(\t\"\x16\n\x08\x43onfigId\x12\n\n\x02id\x18\x01 \x01(\x05\"\xc4\x01\n\x0c\x43onfigUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\x04name\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x19\n\x0c\x63onfig_bytes\x18\x03 \x01(\x0cH\x01\x88\x01\x01\x12,\n\x0b\x63onfig_type\x18\x04 \x01(\x0e\x32\x12.common.ConfigTypeH\x02\x88\x01\x01\x12\x15\n\x08\x63\x61tegory\x18\x05 \x01(\tH\x03\x88\x01\x01\x42\x07\n\x05_nameB\x0f\n\r_config_bytesB\x0e\n\x0c_config_typeB\x0b\n\t_category\"7\n\x11ModelReadResponse\x12\"\n\x06result\x18\x01 \x01(\x0b\x32\x12.model.ModelSchema\"8\n\x11ModelListResponse\x12#\n\x07results\x18\x01 \x03(\x0b\x32\x12.model.ModelSchema\"!\n\x13ModelCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x15\n\x13ModelChangeResponse\"9\n\x12\x43onfigReadResponse\x12#\n\x06result\x18\x01 \x01(\x0b\x32\x13.model.ConfigSchema\":\n\x12\x43onfigListResponse\x12$\n\x07results\x18\x01 \x03(\x0b\x32\x13.model.ConfigSchema\"\"\n\x14\x43onfigCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x16\n\x14\x43onfigChangeResponse2\x9f\x07\n\x0cModelService\x12\x35\n\tReadModel\x12\x0e.model.ModelId\x1a\x18.model.ModelReadResponse\x12=\n\x0fListModelByName\x12\x10.model.ModelName\x1a\x18.model.ModelListResponse\x12\x45\n\x13ListModelByCategory\x12\x14.model.ModelCategory\x1a\x18.model.ModelListResponse\x12M\n\x17ListModelByNameCategory\x12\x18.model.ModelNameCategory\x1a\x18.model.ModelListResponse\x12=\n\x0b\x43reateModel\x12\x12.model.ModelSchema\x1a\x1a.model.ModelCreateResponse\x12=\n\x0bUpdateModel\x12\x12.model.ModelUpdate\x1a\x1a.model.ModelChangeResponse\x12\x39\n\x0b\x44\x65leteModel\x12\x0e.model.ModelId\x1a\x1a.model.ModelChangeResponse\x12=\n\x0c\x41\x64\x64ModelType\x12\x11.model.ModelTypes\x1a\x1a.model.ModelChangeResponse\x12=\n\x0fRemoveModelType\x12\x0e.model.ModelId\x1a\x1a.model.ModelChangeResponse\x12=\n\x0fReadModelConfig\x12\x0f.model.ConfigId\x1a\x19.model.ConfigReadResponse\x12<\n\x0fListModelConfig\x12\x0e.model.ModelId\x1a\x19.model.ConfigListResponse\x12\x45\n\x11\x43reateModelConfig\x12\x13.model.ConfigSchema\x1a\x1b.model.ConfigCreateResponse\x12\x45\n\x11UpdateModelConfig\x12\x13.model.ConfigUpdate\x1a\x1b.model.ConfigChangeResponse\x12\x41\n\x11\x44\x65leteModelConfig\x12\x0f.model.ConfigId\x1a\x1b.model.ConfigChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.model_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_MODELSCHEMA']._serialized_start=73
  _globals['_MODELSCHEMA']._serialized_end=265
  _globals['_CONFIGSCHEMAVEC']._serialized_start=267
  _globals['_CONFIGSCHEMAVEC']._serialized_end=322
  _globals['_MODELID']._serialized_start=324
  _globals['_MODELID']._serialized_end=345
  _globals['_MODELNAME']._serialized_start=347
  _globals['_MODELNAME']._serialized_end=372
  _globals['_MODELCATEGORY']._serialized_start=374
  _globals['_MODELCATEGORY']._serialized_end=407
  _globals['_MODELNAMECATEGORY']._serialized_start=409
  _globals['_MODELNAMECATEGORY']._serialized_end=460
  _globals['_MODELUPDATE']._serialized_start=463
  _globals['_MODELUPDATE']._serialized_end=652
  _globals['_MODELTYPES']._serialized_start=654
  _globals['_MODELTYPES']._serialized_end=711
  _globals['_CONFIGSCHEMA']._serialized_start=714
  _globals['_CONFIGSCHEMA']._serialized_end=868
  _globals['_CONFIGID']._serialized_start=870
  _globals['_CONFIGID']._serialized_end=892
  _globals['_CONFIGUPDATE']._serialized_start=895
  _globals['_CONFIGUPDATE']._serialized_end=1091
  _globals['_MODELREADRESPONSE']._serialized_start=1093
  _globals['_MODELREADRESPONSE']._serialized_end=1148
  _globals['_MODELLISTRESPONSE']._serialized_start=1150
  _globals['_MODELLISTRESPONSE']._serialized_end=1206
  _globals['_MODELCREATERESPONSE']._serialized_start=1208
  _globals['_MODELCREATERESPONSE']._serialized_end=1241
  _globals['_MODELCHANGERESPONSE']._serialized_start=1243
  _globals['_MODELCHANGERESPONSE']._serialized_end=1264
  _globals['_CONFIGREADRESPONSE']._serialized_start=1266
  _globals['_CONFIGREADRESPONSE']._serialized_end=1323
  _globals['_CONFIGLISTRESPONSE']._serialized_start=1325
  _globals['_CONFIGLISTRESPONSE']._serialized_end=1383
  _globals['_CONFIGCREATERESPONSE']._serialized_start=1385
  _globals['_CONFIGCREATERESPONSE']._serialized_end=1419
  _globals['_CONFIGCHANGERESPONSE']._serialized_start=1421
  _globals['_CONFIGCHANGERESPONSE']._serialized_end=1443
  _globals['_MODELSERVICE']._serialized_start=1446
  _globals['_MODELSERVICE']._serialized_end=2373
# @@protoc_insertion_point(module_scope)

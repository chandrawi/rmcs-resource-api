# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: rmcs_resource_api/model.proto
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
    'rmcs_resource_api/model.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1drmcs_resource_api/model.proto\x12\x05model\"\x8a\x01\n\x0bModelSchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x10\n\x08\x63\x61tegory\x18\x02 \x01(\t\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x04 \x01(\t\x12\x11\n\tdata_type\x18\x05 \x03(\r\x12\'\n\x07\x63onfigs\x18\x06 \x03(\x0b\x32\x16.model.ConfigSchemaVec\"7\n\x0f\x43onfigSchemaVec\x12$\n\x07\x63onfigs\x18\x01 \x03(\x0b\x32\x13.model.ConfigSchema\"\x15\n\x07ModelId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x17\n\x08ModelIds\x12\x0b\n\x03ids\x18\x01 \x03(\x0c\"\x14\n\x06TypeId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x19\n\tModelName\x12\x0c\n\x04name\x18\x01 \x01(\t\"!\n\rModelCategory\x12\x10\n\x08\x63\x61tegory\x18\x01 \x01(\t\"o\n\x0bModelOption\x12\x14\n\x07type_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x11\n\x04name\x18\x02 \x01(\tH\x01\x88\x01\x01\x12\x15\n\x08\x63\x61tegory\x18\x03 \x01(\tH\x02\x88\x01\x01\x42\n\n\x08_type_idB\x07\n\x05_nameB\x0b\n\t_category\"\xae\x01\n\x0bModelUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x15\n\x08\x63\x61tegory\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x11\n\x04name\x18\x03 \x01(\tH\x01\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x04 \x01(\tH\x02\x88\x01\x01\x12\x11\n\tdata_type\x18\x05 \x03(\r\x12\x16\n\x0e\x64\x61ta_type_flag\x18\x06 \x01(\x08\x42\x0b\n\t_categoryB\x07\n\x05_nameB\x0e\n\x0c_description\"\x86\x01\n\x0c\x43onfigSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\r\n\x05index\x18\x03 \x01(\x05\x12\x0c\n\x04name\x18\x04 \x01(\t\x12\x14\n\x0c\x63onfig_bytes\x18\x05 \x01(\x0c\x12\x13\n\x0b\x63onfig_type\x18\x06 \x01(\r\x12\x10\n\x08\x63\x61tegory\x18\x07 \x01(\t\"\x16\n\x08\x43onfigId\x12\n\n\x02id\x18\x01 \x01(\x05\"\xb0\x01\n\x0c\x43onfigUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\x04name\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x19\n\x0c\x63onfig_bytes\x18\x03 \x01(\x0cH\x01\x88\x01\x01\x12\x18\n\x0b\x63onfig_type\x18\x04 \x01(\rH\x02\x88\x01\x01\x12\x15\n\x08\x63\x61tegory\x18\x05 \x01(\tH\x03\x88\x01\x01\x42\x07\n\x05_nameB\x0f\n\r_config_bytesB\x0e\n\x0c_config_typeB\x0b\n\t_category\"7\n\x11ModelReadResponse\x12\"\n\x06result\x18\x01 \x01(\x0b\x32\x12.model.ModelSchema\"8\n\x11ModelListResponse\x12#\n\x07results\x18\x01 \x03(\x0b\x32\x12.model.ModelSchema\"!\n\x13ModelCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x15\n\x13ModelChangeResponse\"9\n\x12\x43onfigReadResponse\x12#\n\x06result\x18\x01 \x01(\x0b\x32\x13.model.ConfigSchema\":\n\x12\x43onfigListResponse\x12$\n\x07results\x18\x01 \x03(\x0b\x32\x13.model.ConfigSchema\"\"\n\x14\x43onfigCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x16\n\x14\x43onfigChangeResponse2\x8c\x07\n\x0cModelService\x12\x35\n\tReadModel\x12\x0e.model.ModelId\x1a\x18.model.ModelReadResponse\x12;\n\x0eListModelByIds\x12\x0f.model.ModelIds\x1a\x18.model.ModelListResponse\x12:\n\x0fListModelByType\x12\r.model.TypeId\x1a\x18.model.ModelListResponse\x12=\n\x0fListModelByName\x12\x10.model.ModelName\x1a\x18.model.ModelListResponse\x12\x45\n\x13ListModelByCategory\x12\x14.model.ModelCategory\x1a\x18.model.ModelListResponse\x12?\n\x0fListModelOption\x12\x12.model.ModelOption\x1a\x18.model.ModelListResponse\x12=\n\x0b\x43reateModel\x12\x12.model.ModelSchema\x1a\x1a.model.ModelCreateResponse\x12=\n\x0bUpdateModel\x12\x12.model.ModelUpdate\x1a\x1a.model.ModelChangeResponse\x12\x39\n\x0b\x44\x65leteModel\x12\x0e.model.ModelId\x1a\x1a.model.ModelChangeResponse\x12=\n\x0fReadModelConfig\x12\x0f.model.ConfigId\x1a\x19.model.ConfigReadResponse\x12<\n\x0fListModelConfig\x12\x0e.model.ModelId\x1a\x19.model.ConfigListResponse\x12\x45\n\x11\x43reateModelConfig\x12\x13.model.ConfigSchema\x1a\x1b.model.ConfigCreateResponse\x12\x45\n\x11UpdateModelConfig\x12\x13.model.ConfigUpdate\x1a\x1b.model.ConfigChangeResponse\x12\x41\n\x11\x44\x65leteModelConfig\x12\x0f.model.ConfigId\x1a\x1b.model.ConfigChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.model_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_MODELSCHEMA']._serialized_start=41
  _globals['_MODELSCHEMA']._serialized_end=179
  _globals['_CONFIGSCHEMAVEC']._serialized_start=181
  _globals['_CONFIGSCHEMAVEC']._serialized_end=236
  _globals['_MODELID']._serialized_start=238
  _globals['_MODELID']._serialized_end=259
  _globals['_MODELIDS']._serialized_start=261
  _globals['_MODELIDS']._serialized_end=284
  _globals['_TYPEID']._serialized_start=286
  _globals['_TYPEID']._serialized_end=306
  _globals['_MODELNAME']._serialized_start=308
  _globals['_MODELNAME']._serialized_end=333
  _globals['_MODELCATEGORY']._serialized_start=335
  _globals['_MODELCATEGORY']._serialized_end=368
  _globals['_MODELOPTION']._serialized_start=370
  _globals['_MODELOPTION']._serialized_end=481
  _globals['_MODELUPDATE']._serialized_start=484
  _globals['_MODELUPDATE']._serialized_end=658
  _globals['_CONFIGSCHEMA']._serialized_start=661
  _globals['_CONFIGSCHEMA']._serialized_end=795
  _globals['_CONFIGID']._serialized_start=797
  _globals['_CONFIGID']._serialized_end=819
  _globals['_CONFIGUPDATE']._serialized_start=822
  _globals['_CONFIGUPDATE']._serialized_end=998
  _globals['_MODELREADRESPONSE']._serialized_start=1000
  _globals['_MODELREADRESPONSE']._serialized_end=1055
  _globals['_MODELLISTRESPONSE']._serialized_start=1057
  _globals['_MODELLISTRESPONSE']._serialized_end=1113
  _globals['_MODELCREATERESPONSE']._serialized_start=1115
  _globals['_MODELCREATERESPONSE']._serialized_end=1148
  _globals['_MODELCHANGERESPONSE']._serialized_start=1150
  _globals['_MODELCHANGERESPONSE']._serialized_end=1171
  _globals['_CONFIGREADRESPONSE']._serialized_start=1173
  _globals['_CONFIGREADRESPONSE']._serialized_end=1230
  _globals['_CONFIGLISTRESPONSE']._serialized_start=1232
  _globals['_CONFIGLISTRESPONSE']._serialized_end=1290
  _globals['_CONFIGCREATERESPONSE']._serialized_start=1292
  _globals['_CONFIGCREATERESPONSE']._serialized_end=1326
  _globals['_CONFIGCHANGERESPONSE']._serialized_start=1328
  _globals['_CONFIGCHANGERESPONSE']._serialized_end=1350
  _globals['_MODELSERVICE']._serialized_start=1353
  _globals['_MODELSERVICE']._serialized_end=2261
# @@protoc_insertion_point(module_scope)

# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: rmcs_resource_api/slice.proto
# Protobuf Python Version: 5.29.0
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    29,
    0,
    '',
    'rmcs_resource_api/slice.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1drmcs_resource_api/slice.proto\x12\x05slice\"\x91\x01\n\x0bSliceSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x17\n\x0ftimestamp_begin\x18\x04 \x01(\x03\x12\x15\n\rtimestamp_end\x18\x05 \x01(\x03\x12\x0c\n\x04name\x18\x06 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x07 \x01(\t\"\x15\n\x07SliceId\x12\n\n\x02id\x18\x01 \x01(\x05\"C\n\tSliceTime\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x11\n\ttimestamp\x18\x03 \x01(\x03\"M\n\nSliceRange\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x03 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x04 \x01(\x03\"0\n\rSliceNameTime\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\":\n\x0eSliceNameRange\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\r\n\x05\x62\x65gin\x18\x02 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x03 \x01(\x03\"\xab\x01\n\x0bSliceOption\x12\x16\n\tdevice_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x15\n\x08model_id\x18\x02 \x01(\x0cH\x01\x88\x01\x01\x12\x11\n\x04name\x18\x03 \x01(\tH\x02\x88\x01\x01\x12\x12\n\x05\x62\x65gin\x18\x04 \x01(\x03H\x03\x88\x01\x01\x12\x10\n\x03\x65nd\x18\x05 \x01(\x03H\x04\x88\x01\x01\x42\x0c\n\n_device_idB\x0b\n\t_model_idB\x07\n\x05_nameB\x08\n\x06_beginB\x06\n\x04_end\"\xbf\x01\n\x0bSliceUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x1c\n\x0ftimestamp_begin\x18\x02 \x01(\x03H\x00\x88\x01\x01\x12\x1a\n\rtimestamp_end\x18\x03 \x01(\x03H\x01\x88\x01\x01\x12\x11\n\x04name\x18\x04 \x01(\tH\x02\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x05 \x01(\tH\x03\x88\x01\x01\x42\x12\n\x10_timestamp_beginB\x10\n\x0e_timestamp_endB\x07\n\x05_nameB\x0e\n\x0c_description\"\x7f\n\x0eSliceSetSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x0e\n\x06set_id\x18\x02 \x01(\x0c\x12\x17\n\x0ftimestamp_begin\x18\x03 \x01(\x03\x12\x15\n\rtimestamp_end\x18\x04 \x01(\x03\x12\x0c\n\x04name\x18\x05 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x06 \x01(\t\"1\n\x0cSliceSetTime\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\ttimestamp\x18\x02 \x01(\x03\";\n\rSliceSetRange\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\r\n\x05\x62\x65gin\x18\x02 \x01(\x03\x12\x0b\n\x03\x65nd\x18\x03 \x01(\x03\"\x84\x01\n\x0eSliceSetOption\x12\x13\n\x06set_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x11\n\x04name\x18\x02 \x01(\tH\x01\x88\x01\x01\x12\x12\n\x05\x62\x65gin\x18\x03 \x01(\x03H\x02\x88\x01\x01\x12\x10\n\x03\x65nd\x18\x04 \x01(\x03H\x03\x88\x01\x01\x42\t\n\x07_set_idB\x07\n\x05_nameB\x08\n\x06_beginB\x06\n\x04_end\"7\n\x11SliceReadResponse\x12\"\n\x06result\x18\x01 \x01(\x0b\x32\x12.slice.SliceSchema\"8\n\x11SliceListResponse\x12#\n\x07results\x18\x01 \x03(\x0b\x32\x12.slice.SliceSchema\"!\n\x13SliceCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x15\n\x13SliceChangeResponse\"=\n\x14SliceSetReadResponse\x12%\n\x06result\x18\x01 \x01(\x0b\x32\x15.slice.SliceSetSchema\">\n\x14SliceSetListResponse\x12&\n\x07results\x18\x01 \x03(\x0b\x32\x15.slice.SliceSetSchema2\xd9\t\n\x0cSliceService\x12\x35\n\tReadSlice\x12\x0e.slice.SliceId\x1a\x18.slice.SliceReadResponse\x12=\n\x0fListSliceByTime\x12\x10.slice.SliceTime\x1a\x18.slice.SliceListResponse\x12\x43\n\x14ListSliceByRangeTime\x12\x11.slice.SliceRange\x1a\x18.slice.SliceListResponse\x12\x45\n\x13ListSliceByNameTime\x12\x14.slice.SliceNameTime\x1a\x18.slice.SliceListResponse\x12K\n\x18ListSliceByNameRangeTime\x12\x15.slice.SliceNameRange\x1a\x18.slice.SliceListResponse\x12?\n\x0fListSliceOption\x12\x12.slice.SliceOption\x1a\x18.slice.SliceListResponse\x12=\n\x0b\x43reateSlice\x12\x12.slice.SliceSchema\x1a\x1a.slice.SliceCreateResponse\x12=\n\x0bUpdateSlice\x12\x12.slice.SliceUpdate\x1a\x1a.slice.SliceChangeResponse\x12\x39\n\x0b\x44\x65leteSlice\x12\x0e.slice.SliceId\x1a\x1a.slice.SliceChangeResponse\x12;\n\x0cReadSliceSet\x12\x0e.slice.SliceId\x1a\x1b.slice.SliceSetReadResponse\x12\x46\n\x12ListSliceSetByTime\x12\x13.slice.SliceSetTime\x1a\x1b.slice.SliceSetListResponse\x12L\n\x17ListSliceSetByRangeTime\x12\x14.slice.SliceSetRange\x1a\x1b.slice.SliceSetListResponse\x12K\n\x16ListSliceSetByNameTime\x12\x14.slice.SliceNameTime\x1a\x1b.slice.SliceSetListResponse\x12Q\n\x1bListSliceSetByNameRangeTime\x12\x15.slice.SliceNameRange\x1a\x1b.slice.SliceSetListResponse\x12H\n\x12ListSliceSetOption\x12\x15.slice.SliceSetOption\x1a\x1b.slice.SliceSetListResponse\x12\x43\n\x0e\x43reateSliceSet\x12\x15.slice.SliceSetSchema\x1a\x1a.slice.SliceCreateResponse\x12@\n\x0eUpdateSliceSet\x12\x12.slice.SliceUpdate\x1a\x1a.slice.SliceChangeResponse\x12<\n\x0e\x44\x65leteSliceSet\x12\x0e.slice.SliceId\x1a\x1a.slice.SliceChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.slice_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_SLICESCHEMA']._serialized_start=41
  _globals['_SLICESCHEMA']._serialized_end=186
  _globals['_SLICEID']._serialized_start=188
  _globals['_SLICEID']._serialized_end=209
  _globals['_SLICETIME']._serialized_start=211
  _globals['_SLICETIME']._serialized_end=278
  _globals['_SLICERANGE']._serialized_start=280
  _globals['_SLICERANGE']._serialized_end=357
  _globals['_SLICENAMETIME']._serialized_start=359
  _globals['_SLICENAMETIME']._serialized_end=407
  _globals['_SLICENAMERANGE']._serialized_start=409
  _globals['_SLICENAMERANGE']._serialized_end=467
  _globals['_SLICEOPTION']._serialized_start=470
  _globals['_SLICEOPTION']._serialized_end=641
  _globals['_SLICEUPDATE']._serialized_start=644
  _globals['_SLICEUPDATE']._serialized_end=835
  _globals['_SLICESETSCHEMA']._serialized_start=837
  _globals['_SLICESETSCHEMA']._serialized_end=964
  _globals['_SLICESETTIME']._serialized_start=966
  _globals['_SLICESETTIME']._serialized_end=1015
  _globals['_SLICESETRANGE']._serialized_start=1017
  _globals['_SLICESETRANGE']._serialized_end=1076
  _globals['_SLICESETOPTION']._serialized_start=1079
  _globals['_SLICESETOPTION']._serialized_end=1211
  _globals['_SLICEREADRESPONSE']._serialized_start=1213
  _globals['_SLICEREADRESPONSE']._serialized_end=1268
  _globals['_SLICELISTRESPONSE']._serialized_start=1270
  _globals['_SLICELISTRESPONSE']._serialized_end=1326
  _globals['_SLICECREATERESPONSE']._serialized_start=1328
  _globals['_SLICECREATERESPONSE']._serialized_end=1361
  _globals['_SLICECHANGERESPONSE']._serialized_start=1363
  _globals['_SLICECHANGERESPONSE']._serialized_end=1384
  _globals['_SLICESETREADRESPONSE']._serialized_start=1386
  _globals['_SLICESETREADRESPONSE']._serialized_end=1447
  _globals['_SLICESETLISTRESPONSE']._serialized_start=1449
  _globals['_SLICESETLISTRESPONSE']._serialized_end=1511
  _globals['_SLICESERVICE']._serialized_start=1514
  _globals['_SLICESERVICE']._serialized_end=2755
# @@protoc_insertion_point(module_scope)

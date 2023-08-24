# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rmcs_resource_api/slice.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1drmcs_resource_api/slice.proto\x12\x05slice\"\xb9\x01\n\x0bSliceSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x17\n\x0ftimestamp_begin\x18\x04 \x01(\x03\x12\x15\n\rtimestamp_end\x18\x05 \x01(\x03\x12\x13\n\x0bindex_begin\x18\x06 \x01(\x05\x12\x11\n\tindex_end\x18\x07 \x01(\x05\x12\x0c\n\x04name\x18\x08 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\t \x01(\t\"\x15\n\x07SliceId\x12\n\n\x02id\x18\x01 \x01(\x05\"\x19\n\tSliceName\x12\x0c\n\x04name\x18\x01 \x01(\t\" \n\x0bSliceDevice\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\"\x1e\n\nSliceModel\x12\x10\n\x08model_id\x18\x01 \x01(\x0c\"7\n\x10SliceDeviceModel\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\"\x8f\x02\n\x0bSliceUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x1c\n\x0ftimestamp_begin\x18\x02 \x01(\x03H\x00\x88\x01\x01\x12\x1a\n\rtimestamp_end\x18\x03 \x01(\x03H\x01\x88\x01\x01\x12\x18\n\x0bindex_begin\x18\x04 \x01(\x05H\x02\x88\x01\x01\x12\x16\n\tindex_end\x18\x05 \x01(\x05H\x03\x88\x01\x01\x12\x11\n\x04name\x18\x06 \x01(\tH\x04\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x07 \x01(\tH\x05\x88\x01\x01\x42\x12\n\x10_timestamp_beginB\x10\n\x0e_timestamp_endB\x0e\n\x0c_index_beginB\x0c\n\n_index_endB\x07\n\x05_nameB\x0e\n\x0c_description\"7\n\x11SliceReadResponse\x12\"\n\x06result\x18\x01 \x01(\x0b\x32\x12.slice.SliceSchema\"8\n\x11SliceListResponse\x12#\n\x07results\x18\x01 \x03(\x0b\x32\x12.slice.SliceSchema\"!\n\x13SliceCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x15\n\x13SliceChangeResponse2\x8e\x04\n\x0cSliceService\x12\x35\n\tReadSlice\x12\x0e.slice.SliceId\x1a\x18.slice.SliceReadResponse\x12=\n\x0fListSliceByName\x12\x10.slice.SliceName\x1a\x18.slice.SliceListResponse\x12\x41\n\x11ListSliceByDevice\x12\x12.slice.SliceDevice\x1a\x18.slice.SliceListResponse\x12?\n\x10ListSliceByModel\x12\x11.slice.SliceModel\x1a\x18.slice.SliceListResponse\x12K\n\x16ListSliceByDeviceModel\x12\x17.slice.SliceDeviceModel\x1a\x18.slice.SliceListResponse\x12=\n\x0b\x43reateSlice\x12\x12.slice.SliceSchema\x1a\x1a.slice.SliceCreateResponse\x12=\n\x0bUpdateSlice\x12\x12.slice.SliceUpdate\x1a\x1a.slice.SliceChangeResponse\x12\x39\n\x0b\x44\x65leteSlice\x12\x0e.slice.SliceId\x1a\x1a.slice.SliceChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.slice_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_SLICESCHEMA']._serialized_start=41
  _globals['_SLICESCHEMA']._serialized_end=226
  _globals['_SLICEID']._serialized_start=228
  _globals['_SLICEID']._serialized_end=249
  _globals['_SLICENAME']._serialized_start=251
  _globals['_SLICENAME']._serialized_end=276
  _globals['_SLICEDEVICE']._serialized_start=278
  _globals['_SLICEDEVICE']._serialized_end=310
  _globals['_SLICEMODEL']._serialized_start=312
  _globals['_SLICEMODEL']._serialized_end=342
  _globals['_SLICEDEVICEMODEL']._serialized_start=344
  _globals['_SLICEDEVICEMODEL']._serialized_end=399
  _globals['_SLICEUPDATE']._serialized_start=402
  _globals['_SLICEUPDATE']._serialized_end=673
  _globals['_SLICEREADRESPONSE']._serialized_start=675
  _globals['_SLICEREADRESPONSE']._serialized_end=730
  _globals['_SLICELISTRESPONSE']._serialized_start=732
  _globals['_SLICELISTRESPONSE']._serialized_end=788
  _globals['_SLICECREATERESPONSE']._serialized_start=790
  _globals['_SLICECREATERESPONSE']._serialized_end=823
  _globals['_SLICECHANGERESPONSE']._serialized_start=825
  _globals['_SLICECHANGERESPONSE']._serialized_end=846
  _globals['_SLICESERVICE']._serialized_start=849
  _globals['_SLICESERVICE']._serialized_end=1375
# @@protoc_insertion_point(module_scope)

# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: rmcs_resource_api/set.proto
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
    'rmcs_resource_api/set.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1brmcs_resource_api/set.proto\x12\x03set\"p\n\tSetSchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x13\n\x0btemplate_id\x18\x02 \x01(\x0c\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x04 \x01(\t\x12\x1f\n\x07members\x18\x05 \x03(\x0b\x32\x0e.set.SetMember\"D\n\tSetMember\x12\x11\n\tdevice_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x12\n\ndata_index\x18\x03 \x01(\x0c\"\x13\n\x05SetId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x15\n\x06SetIds\x12\x0b\n\x03ids\x18\x01 \x03(\x0c\"\x17\n\x07SetName\x12\x0c\n\x04name\x18\x01 \x01(\t\"Q\n\tSetOption\x12\x18\n\x0btemplate_id\x18\x01 \x01(\x0cH\x00\x88\x01\x01\x12\x11\n\x04name\x18\x02 \x01(\tH\x01\x88\x01\x01\x42\x0e\n\x0c_template_idB\x07\n\x05_name\"\x87\x01\n\tSetUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x18\n\x0btemplate_id\x18\x02 \x01(\x0cH\x00\x88\x01\x01\x12\x11\n\x04name\x18\x03 \x01(\tH\x01\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x04 \x01(\tH\x02\x88\x01\x01\x42\x0e\n\x0c_template_idB\x07\n\x05_nameB\x0e\n\x0c_description\"[\n\x10SetMemberRequest\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x12\n\ndata_index\x18\x04 \x01(\x0c\"q\n\rSetMemberSwap\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x13\n\x0b\x64\x65vice_id_1\x18\x02 \x01(\x0c\x12\x12\n\nmodel_id_1\x18\x03 \x01(\x0c\x12\x13\n\x0b\x64\x65vice_id_2\x18\x04 \x01(\x0c\x12\x12\n\nmodel_id_2\x18\x05 \x01(\x0c\"k\n\x11SetTemplateSchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12\'\n\x07members\x18\x05 \x03(\x0b\x32\x16.set.SetTemplateMember\"J\n\x11SetTemplateMember\x12\x0f\n\x07type_id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\x12\x12\n\ndata_index\x18\x03 \x01(\x0c\"\x1b\n\rSetTemplateId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x1d\n\x0eSetTemplateIds\x12\x0b\n\x03ids\x18\x01 \x03(\x0c\"\x1f\n\x0fSetTemplateName\x12\x0c\n\x04name\x18\x01 \x01(\t\"/\n\x11SetTemplateOption\x12\x11\n\x04name\x18\x01 \x01(\tH\x00\x88\x01\x01\x42\x07\n\x05_name\"e\n\x11SetTemplateUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x11\n\x04name\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x03 \x01(\tH\x01\x88\x01\x01\x42\x07\n\x05_nameB\x0e\n\x0c_description\"y\n\x18SetTemplateMemberRequest\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x0f\n\x07type_id\x18\x02 \x01(\x0c\x12\x10\n\x08model_id\x18\x03 \x01(\x0c\x12\x12\n\ndata_index\x18\x04 \x01(\x0c\x12\x16\n\x0etemplate_index\x18\x05 \x01(\x05\"[\n\x15SetTemplateMemberSwap\x12\x0e\n\x06set_id\x18\x01 \x01(\x0c\x12\x18\n\x10template_index_1\x18\x02 \x01(\x05\x12\x18\n\x10template_index_2\x18\x03 \x01(\x05\"1\n\x0fSetReadResponse\x12\x1e\n\x06result\x18\x01 \x01(\x0b\x32\x0e.set.SetSchema\"2\n\x0fSetListResponse\x12\x1f\n\x07results\x18\x01 \x03(\x0b\x32\x0e.set.SetSchema\"\x1f\n\x11SetCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x13\n\x11SetChangeResponse\">\n\x14TemplateReadResponse\x12&\n\x06result\x18\x01 \x01(\x0b\x32\x16.set.SetTemplateSchema\"?\n\x14TemplateListResponse\x12\'\n\x07results\x18\x01 \x03(\x0b\x32\x16.set.SetTemplateSchema\"$\n\x16TemplateCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x18\n\x16TemplateChangeResponse2\xe7\n\n\nSetService\x12+\n\x07ReadSet\x12\n.set.SetId\x1a\x14.set.SetReadResponse\x12\x31\n\x0cListSetByIds\x12\x0b.set.SetIds\x1a\x14.set.SetListResponse\x12=\n\x11ListSetByTemplate\x12\x12.set.SetTemplateId\x1a\x14.set.SetListResponse\x12\x33\n\rListSetByName\x12\x0c.set.SetName\x1a\x14.set.SetListResponse\x12\x35\n\rListSetOption\x12\x0e.set.SetOption\x1a\x14.set.SetListResponse\x12\x33\n\tCreateSet\x12\x0e.set.SetSchema\x1a\x16.set.SetCreateResponse\x12\x33\n\tUpdateSet\x12\x0e.set.SetUpdate\x1a\x16.set.SetChangeResponse\x12/\n\tDeleteSet\x12\n.set.SetId\x1a\x16.set.SetChangeResponse\x12=\n\x0c\x41\x64\x64SetMember\x12\x15.set.SetMemberRequest\x1a\x16.set.SetChangeResponse\x12@\n\x0fRemoveSetMember\x12\x15.set.SetMemberRequest\x1a\x16.set.SetChangeResponse\x12;\n\rSwapSetMember\x12\x12.set.SetMemberSwap\x1a\x16.set.SetChangeResponse\x12@\n\x0fReadSetTemplate\x12\x12.set.SetTemplateId\x1a\x19.set.TemplateReadResponse\x12\x46\n\x14ListSetTemplateByIds\x12\x13.set.SetTemplateIds\x1a\x19.set.TemplateListResponse\x12H\n\x15ListSetTemplateByName\x12\x14.set.SetTemplateName\x1a\x19.set.TemplateListResponse\x12J\n\x15ListSetTemplateOption\x12\x16.set.SetTemplateOption\x1a\x19.set.TemplateListResponse\x12H\n\x11\x43reateSetTemplate\x12\x16.set.SetTemplateSchema\x1a\x1b.set.TemplateCreateResponse\x12H\n\x11UpdateSetTemplate\x12\x16.set.SetTemplateUpdate\x1a\x1b.set.TemplateChangeResponse\x12\x44\n\x11\x44\x65leteSetTemplate\x12\x12.set.SetTemplateId\x1a\x1b.set.TemplateChangeResponse\x12R\n\x14\x41\x64\x64SetTemplateMember\x12\x1d.set.SetTemplateMemberRequest\x1a\x1b.set.TemplateChangeResponse\x12U\n\x17RemoveSetTemplateMember\x12\x1d.set.SetTemplateMemberRequest\x1a\x1b.set.TemplateChangeResponse\x12P\n\x15SwapSetTemplateMember\x12\x1a.set.SetTemplateMemberSwap\x1a\x1b.set.TemplateChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rmcs_resource_api.set_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_SETSCHEMA']._serialized_start=36
  _globals['_SETSCHEMA']._serialized_end=148
  _globals['_SETMEMBER']._serialized_start=150
  _globals['_SETMEMBER']._serialized_end=218
  _globals['_SETID']._serialized_start=220
  _globals['_SETID']._serialized_end=239
  _globals['_SETIDS']._serialized_start=241
  _globals['_SETIDS']._serialized_end=262
  _globals['_SETNAME']._serialized_start=264
  _globals['_SETNAME']._serialized_end=287
  _globals['_SETOPTION']._serialized_start=289
  _globals['_SETOPTION']._serialized_end=370
  _globals['_SETUPDATE']._serialized_start=373
  _globals['_SETUPDATE']._serialized_end=508
  _globals['_SETMEMBERREQUEST']._serialized_start=510
  _globals['_SETMEMBERREQUEST']._serialized_end=601
  _globals['_SETMEMBERSWAP']._serialized_start=603
  _globals['_SETMEMBERSWAP']._serialized_end=716
  _globals['_SETTEMPLATESCHEMA']._serialized_start=718
  _globals['_SETTEMPLATESCHEMA']._serialized_end=825
  _globals['_SETTEMPLATEMEMBER']._serialized_start=827
  _globals['_SETTEMPLATEMEMBER']._serialized_end=901
  _globals['_SETTEMPLATEID']._serialized_start=903
  _globals['_SETTEMPLATEID']._serialized_end=930
  _globals['_SETTEMPLATEIDS']._serialized_start=932
  _globals['_SETTEMPLATEIDS']._serialized_end=961
  _globals['_SETTEMPLATENAME']._serialized_start=963
  _globals['_SETTEMPLATENAME']._serialized_end=994
  _globals['_SETTEMPLATEOPTION']._serialized_start=996
  _globals['_SETTEMPLATEOPTION']._serialized_end=1043
  _globals['_SETTEMPLATEUPDATE']._serialized_start=1045
  _globals['_SETTEMPLATEUPDATE']._serialized_end=1146
  _globals['_SETTEMPLATEMEMBERREQUEST']._serialized_start=1148
  _globals['_SETTEMPLATEMEMBERREQUEST']._serialized_end=1269
  _globals['_SETTEMPLATEMEMBERSWAP']._serialized_start=1271
  _globals['_SETTEMPLATEMEMBERSWAP']._serialized_end=1362
  _globals['_SETREADRESPONSE']._serialized_start=1364
  _globals['_SETREADRESPONSE']._serialized_end=1413
  _globals['_SETLISTRESPONSE']._serialized_start=1415
  _globals['_SETLISTRESPONSE']._serialized_end=1465
  _globals['_SETCREATERESPONSE']._serialized_start=1467
  _globals['_SETCREATERESPONSE']._serialized_end=1498
  _globals['_SETCHANGERESPONSE']._serialized_start=1500
  _globals['_SETCHANGERESPONSE']._serialized_end=1519
  _globals['_TEMPLATEREADRESPONSE']._serialized_start=1521
  _globals['_TEMPLATEREADRESPONSE']._serialized_end=1583
  _globals['_TEMPLATELISTRESPONSE']._serialized_start=1585
  _globals['_TEMPLATELISTRESPONSE']._serialized_end=1648
  _globals['_TEMPLATECREATERESPONSE']._serialized_start=1650
  _globals['_TEMPLATECREATERESPONSE']._serialized_end=1686
  _globals['_TEMPLATECHANGERESPONSE']._serialized_start=1688
  _globals['_TEMPLATECHANGERESPONSE']._serialized_end=1712
  _globals['_SETSERVICE']._serialized_start=1715
  _globals['_SETSERVICE']._serialized_end=3098
# @@protoc_insertion_point(module_scope)

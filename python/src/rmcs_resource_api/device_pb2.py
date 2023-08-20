# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: device.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


import common_pb2 as common__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0c\x64\x65vice.proto\x12\x06\x64\x65vice\x1a\x0c\x63ommon.proto\"\xb8\x01\n\x0c\x44\x65viceSchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x12\n\ngateway_id\x18\x02 \x01(\x0c\x12\x15\n\rserial_number\x18\x03 \x01(\t\x12\x0c\n\x04name\x18\x04 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x05 \x01(\t\x12\'\n\x0b\x64\x65vice_type\x18\x06 \x01(\x0b\x32\x12.device.TypeSchema\x12%\n\x07\x63onfigs\x18\x07 \x03(\x0b\x32\x14.device.ConfigSchema\"\xa6\x01\n\rGatewaySchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x15\n\rserial_number\x18\x02 \x01(\t\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x04 \x01(\t\x12(\n\x0cgateway_type\x18\x05 \x01(\x0b\x32\x12.device.TypeSchema\x12%\n\x07\x63onfigs\x18\x06 \x03(\x0b\x32\x14.device.ConfigSchema\"\x16\n\x08\x44\x65viceId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x17\n\tGatewayId\x12\n\n\x02id\x18\x01 \x01(\x0c\"%\n\x0cSerialNumber\x12\x15\n\rserial_number\x18\x01 \x01(\t\"\x1a\n\nDeviceName\x12\x0c\n\x04name\x18\x01 \x01(\t\"\x1b\n\x0bGatewayName\x12\x0c\n\x04name\x18\x01 \x01(\t\"8\n\x11\x44\x65viceGatewayType\x12\x12\n\ngateway_id\x18\x01 \x01(\x0c\x12\x0f\n\x07type_id\x18\x02 \x01(\x0c\"5\n\x11\x44\x65viceGatewayName\x12\x12\n\ngateway_id\x18\x01 \x01(\x0c\x12\x0c\n\x04name\x18\x02 \x01(\t\"\xd8\x01\n\x0c\x44\x65viceUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x17\n\ngateway_id\x18\x02 \x01(\x0cH\x00\x88\x01\x01\x12\x1a\n\rserial_number\x18\x03 \x01(\tH\x01\x88\x01\x01\x12\x11\n\x04name\x18\x04 \x01(\tH\x02\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x05 \x01(\tH\x03\x88\x01\x01\x12\x14\n\x07type_id\x18\x06 \x01(\x0cH\x04\x88\x01\x01\x42\r\n\x0b_gateway_idB\x10\n\x0e_serial_numberB\x07\n\x05_nameB\x0e\n\x0c_descriptionB\n\n\x08_type_id\"\xb1\x01\n\rGatewayUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x1a\n\rserial_number\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x11\n\x04name\x18\x03 \x01(\tH\x01\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x04 \x01(\tH\x02\x88\x01\x01\x12\x14\n\x07type_id\x18\x05 \x01(\x0cH\x03\x88\x01\x01\x42\x10\n\x0e_serial_numberB\x07\n\x05_nameB\x0e\n\x0c_descriptionB\n\n\x08_type_id\"\x8c\x01\n\x0c\x43onfigSchema\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\tdevice_id\x18\x02 \x01(\x0c\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x14\n\x0c\x63onfig_bytes\x18\x04 \x01(\x0c\x12\'\n\x0b\x63onfig_type\x18\x05 \x01(\x0e\x32\x12.common.ConfigType\x12\x10\n\x08\x63\x61tegory\x18\x06 \x01(\t\"\x16\n\x08\x43onfigId\x12\n\n\x02id\x18\x01 \x01(\x05\"\xc4\x01\n\x0c\x43onfigUpdate\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x11\n\x04name\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x19\n\x0c\x63onfig_bytes\x18\x03 \x01(\x0cH\x01\x88\x01\x01\x12,\n\x0b\x63onfig_type\x18\x04 \x01(\x0e\x32\x12.common.ConfigTypeH\x02\x88\x01\x01\x12\x15\n\x08\x63\x61tegory\x18\x05 \x01(\tH\x03\x88\x01\x01\x42\x07\n\x05_nameB\x0f\n\r_config_bytesB\x0e\n\x0c_config_typeB\x0b\n\t_category\"K\n\nTypeSchema\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12\x0e\n\x06models\x18\x04 \x03(\x0c\"\x14\n\x06TypeId\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x18\n\x08TypeName\x12\x0c\n\x04name\x18\x01 \x01(\t\"^\n\nTypeUpdate\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x11\n\x04name\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x18\n\x0b\x64\x65scription\x18\x03 \x01(\tH\x01\x88\x01\x01\x42\x07\n\x05_nameB\x0e\n\x0c_description\")\n\tTypeModel\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x10\n\x08model_id\x18\x02 \x01(\x0c\":\n\x12\x44\x65viceReadResponse\x12$\n\x06result\x18\x01 \x01(\x0b\x32\x14.device.DeviceSchema\";\n\x12\x44\x65viceListResponse\x12%\n\x07results\x18\x01 \x03(\x0b\x32\x14.device.DeviceSchema\"\x16\n\x14\x44\x65viceChangeResponse\"<\n\x13GatewayReadResponse\x12%\n\x06result\x18\x01 \x01(\x0b\x32\x15.device.GatewaySchema\"=\n\x13GatewayListResponse\x12&\n\x07results\x18\x01 \x03(\x0b\x32\x15.device.GatewaySchema\"\x17\n\x15GatewayChangeResponse\":\n\x12\x43onfigReadResponse\x12$\n\x06result\x18\x01 \x01(\x0b\x32\x14.device.ConfigSchema\";\n\x12\x43onfigListResponse\x12%\n\x07results\x18\x01 \x03(\x0b\x32\x14.device.ConfigSchema\"\"\n\x14\x43onfigCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x05\"\x16\n\x14\x43onfigChangeResponse\"6\n\x10TypeReadResponse\x12\"\n\x06result\x18\x01 \x01(\x0b\x32\x12.device.TypeSchema\"7\n\x10TypeListResponse\x12#\n\x07results\x18\x01 \x03(\x0b\x32\x12.device.TypeSchema\" \n\x12TypeCreateResponse\x12\n\n\x02id\x18\x01 \x01(\x0c\"\x14\n\x12TypeChangeResponse2\x91\x12\n\rDeviceService\x12:\n\nReadDevice\x12\x10.device.DeviceId\x1a\x1a.device.DeviceReadResponse\x12\x42\n\x0eReadDeviceBySn\x12\x14.device.SerialNumber\x1a\x1a.device.DeviceReadResponse\x12\x44\n\x13ListDeviceByGateway\x12\x11.device.GatewayId\x1a\x1a.device.DeviceListResponse\x12>\n\x10ListDeviceByType\x12\x0e.device.TypeId\x1a\x1a.device.DeviceListResponse\x12\x42\n\x10ListDeviceByName\x12\x12.device.DeviceName\x1a\x1a.device.DeviceListResponse\x12P\n\x17ListDeviceByGatewayType\x12\x19.device.DeviceGatewayType\x1a\x1a.device.DeviceListResponse\x12P\n\x17ListDeviceByGatewayName\x12\x19.device.DeviceGatewayName\x1a\x1a.device.DeviceListResponse\x12\x42\n\x0c\x43reateDevice\x12\x14.device.DeviceSchema\x1a\x1c.device.DeviceChangeResponse\x12\x42\n\x0cUpdateDevice\x12\x14.device.DeviceUpdate\x1a\x1c.device.DeviceChangeResponse\x12>\n\x0c\x44\x65leteDevice\x12\x10.device.DeviceId\x1a\x1c.device.DeviceChangeResponse\x12=\n\x0bReadGateway\x12\x11.device.GatewayId\x1a\x1b.device.GatewayReadResponse\x12\x44\n\x0fReadGatewayBySn\x12\x14.device.SerialNumber\x1a\x1b.device.GatewayReadResponse\x12@\n\x11ListGatewayByType\x12\x0e.device.TypeId\x1a\x1b.device.GatewayListResponse\x12\x45\n\x11ListGatewayByName\x12\x13.device.GatewayName\x1a\x1b.device.GatewayListResponse\x12\x45\n\rCreateGateway\x12\x15.device.GatewaySchema\x1a\x1d.device.GatewayChangeResponse\x12\x45\n\rUpdateGateway\x12\x15.device.GatewayUpdate\x1a\x1d.device.GatewayChangeResponse\x12\x41\n\rDeleteGateway\x12\x11.device.GatewayId\x1a\x1d.device.GatewayChangeResponse\x12@\n\x10ReadDeviceConfig\x12\x10.device.ConfigId\x1a\x1a.device.ConfigReadResponse\x12@\n\x10ListDeviceConfig\x12\x10.device.DeviceId\x1a\x1a.device.ConfigListResponse\x12H\n\x12\x43reateDeviceConfig\x12\x14.device.ConfigSchema\x1a\x1c.device.ConfigCreateResponse\x12H\n\x12UpdateDeviceConfig\x12\x14.device.ConfigUpdate\x1a\x1c.device.ConfigChangeResponse\x12\x44\n\x12\x44\x65leteDeviceConfig\x12\x10.device.ConfigId\x1a\x1c.device.ConfigChangeResponse\x12\x41\n\x11ReadGatewayConfig\x12\x10.device.ConfigId\x1a\x1a.device.ConfigReadResponse\x12\x42\n\x11ListGatewayConfig\x12\x11.device.GatewayId\x1a\x1a.device.ConfigListResponse\x12I\n\x13\x43reateGatewayConfig\x12\x14.device.ConfigSchema\x1a\x1c.device.ConfigCreateResponse\x12I\n\x13UpdateGatewayConfig\x12\x14.device.ConfigUpdate\x1a\x1c.device.ConfigChangeResponse\x12\x45\n\x13\x44\x65leteGatewayConfig\x12\x10.device.ConfigId\x1a\x1c.device.ConfigChangeResponse\x12\x34\n\x08ReadType\x12\x0e.device.TypeId\x1a\x18.device.TypeReadResponse\x12<\n\x0eListTypeByName\x12\x10.device.TypeName\x1a\x18.device.TypeListResponse\x12<\n\nCreateType\x12\x12.device.TypeSchema\x1a\x1a.device.TypeCreateResponse\x12<\n\nUpdateType\x12\x12.device.TypeUpdate\x1a\x1a.device.TypeChangeResponse\x12\x38\n\nDeleteType\x12\x0e.device.TypeId\x1a\x1a.device.TypeChangeResponse\x12=\n\x0c\x41\x64\x64TypeModel\x12\x11.device.TypeModel\x1a\x1a.device.TypeChangeResponse\x12@\n\x0fRemoveTypeModel\x12\x11.device.TypeModel\x1a\x1a.device.TypeChangeResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'device_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_DEVICESCHEMA']._serialized_start=39
  _globals['_DEVICESCHEMA']._serialized_end=223
  _globals['_GATEWAYSCHEMA']._serialized_start=226
  _globals['_GATEWAYSCHEMA']._serialized_end=392
  _globals['_DEVICEID']._serialized_start=394
  _globals['_DEVICEID']._serialized_end=416
  _globals['_GATEWAYID']._serialized_start=418
  _globals['_GATEWAYID']._serialized_end=441
  _globals['_SERIALNUMBER']._serialized_start=443
  _globals['_SERIALNUMBER']._serialized_end=480
  _globals['_DEVICENAME']._serialized_start=482
  _globals['_DEVICENAME']._serialized_end=508
  _globals['_GATEWAYNAME']._serialized_start=510
  _globals['_GATEWAYNAME']._serialized_end=537
  _globals['_DEVICEGATEWAYTYPE']._serialized_start=539
  _globals['_DEVICEGATEWAYTYPE']._serialized_end=595
  _globals['_DEVICEGATEWAYNAME']._serialized_start=597
  _globals['_DEVICEGATEWAYNAME']._serialized_end=650
  _globals['_DEVICEUPDATE']._serialized_start=653
  _globals['_DEVICEUPDATE']._serialized_end=869
  _globals['_GATEWAYUPDATE']._serialized_start=872
  _globals['_GATEWAYUPDATE']._serialized_end=1049
  _globals['_CONFIGSCHEMA']._serialized_start=1052
  _globals['_CONFIGSCHEMA']._serialized_end=1192
  _globals['_CONFIGID']._serialized_start=1194
  _globals['_CONFIGID']._serialized_end=1216
  _globals['_CONFIGUPDATE']._serialized_start=1219
  _globals['_CONFIGUPDATE']._serialized_end=1415
  _globals['_TYPESCHEMA']._serialized_start=1417
  _globals['_TYPESCHEMA']._serialized_end=1492
  _globals['_TYPEID']._serialized_start=1494
  _globals['_TYPEID']._serialized_end=1514
  _globals['_TYPENAME']._serialized_start=1516
  _globals['_TYPENAME']._serialized_end=1540
  _globals['_TYPEUPDATE']._serialized_start=1542
  _globals['_TYPEUPDATE']._serialized_end=1636
  _globals['_TYPEMODEL']._serialized_start=1638
  _globals['_TYPEMODEL']._serialized_end=1679
  _globals['_DEVICEREADRESPONSE']._serialized_start=1681
  _globals['_DEVICEREADRESPONSE']._serialized_end=1739
  _globals['_DEVICELISTRESPONSE']._serialized_start=1741
  _globals['_DEVICELISTRESPONSE']._serialized_end=1800
  _globals['_DEVICECHANGERESPONSE']._serialized_start=1802
  _globals['_DEVICECHANGERESPONSE']._serialized_end=1824
  _globals['_GATEWAYREADRESPONSE']._serialized_start=1826
  _globals['_GATEWAYREADRESPONSE']._serialized_end=1886
  _globals['_GATEWAYLISTRESPONSE']._serialized_start=1888
  _globals['_GATEWAYLISTRESPONSE']._serialized_end=1949
  _globals['_GATEWAYCHANGERESPONSE']._serialized_start=1951
  _globals['_GATEWAYCHANGERESPONSE']._serialized_end=1974
  _globals['_CONFIGREADRESPONSE']._serialized_start=1976
  _globals['_CONFIGREADRESPONSE']._serialized_end=2034
  _globals['_CONFIGLISTRESPONSE']._serialized_start=2036
  _globals['_CONFIGLISTRESPONSE']._serialized_end=2095
  _globals['_CONFIGCREATERESPONSE']._serialized_start=2097
  _globals['_CONFIGCREATERESPONSE']._serialized_end=2131
  _globals['_CONFIGCHANGERESPONSE']._serialized_start=2133
  _globals['_CONFIGCHANGERESPONSE']._serialized_end=2155
  _globals['_TYPEREADRESPONSE']._serialized_start=2157
  _globals['_TYPEREADRESPONSE']._serialized_end=2211
  _globals['_TYPELISTRESPONSE']._serialized_start=2213
  _globals['_TYPELISTRESPONSE']._serialized_end=2268
  _globals['_TYPECREATERESPONSE']._serialized_start=2270
  _globals['_TYPECREATERESPONSE']._serialized_end=2302
  _globals['_TYPECHANGERESPONSE']._serialized_start=2304
  _globals['_TYPECHANGERESPONSE']._serialized_end=2324
  _globals['_DEVICESERVICE']._serialized_start=2327
  _globals['_DEVICESERVICE']._serialized_end=4648
# @@protoc_insertion_point(module_scope)

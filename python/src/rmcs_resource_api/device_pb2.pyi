from rmcs_resource_api import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class DeviceSchema(_message.Message):
    __slots__ = ("id", "gateway_id", "serial_number", "name", "description", "device_type", "configs")
    ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    SERIAL_NUMBER_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DEVICE_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONFIGS_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    gateway_id: bytes
    serial_number: str
    name: str
    description: str
    device_type: TypeSchema
    configs: _containers.RepeatedCompositeFieldContainer[ConfigSchema]
    def __init__(self, id: _Optional[bytes] = ..., gateway_id: _Optional[bytes] = ..., serial_number: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., device_type: _Optional[_Union[TypeSchema, _Mapping]] = ..., configs: _Optional[_Iterable[_Union[ConfigSchema, _Mapping]]] = ...) -> None: ...

class GatewaySchema(_message.Message):
    __slots__ = ("id", "serial_number", "name", "description", "gateway_type", "configs")
    ID_FIELD_NUMBER: _ClassVar[int]
    SERIAL_NUMBER_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONFIGS_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    serial_number: str
    name: str
    description: str
    gateway_type: TypeSchema
    configs: _containers.RepeatedCompositeFieldContainer[ConfigSchema]
    def __init__(self, id: _Optional[bytes] = ..., serial_number: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., gateway_type: _Optional[_Union[TypeSchema, _Mapping]] = ..., configs: _Optional[_Iterable[_Union[ConfigSchema, _Mapping]]] = ...) -> None: ...

class DeviceId(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class GatewayId(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class SerialNumber(_message.Message):
    __slots__ = ("serial_number",)
    SERIAL_NUMBER_FIELD_NUMBER: _ClassVar[int]
    serial_number: str
    def __init__(self, serial_number: _Optional[str] = ...) -> None: ...

class DeviceName(_message.Message):
    __slots__ = ("name",)
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class GatewayName(_message.Message):
    __slots__ = ("name",)
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class DeviceGatewayType(_message.Message):
    __slots__ = ("gateway_id", "type_id")
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    TYPE_ID_FIELD_NUMBER: _ClassVar[int]
    gateway_id: bytes
    type_id: bytes
    def __init__(self, gateway_id: _Optional[bytes] = ..., type_id: _Optional[bytes] = ...) -> None: ...

class DeviceGatewayName(_message.Message):
    __slots__ = ("gateway_id", "name")
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    gateway_id: bytes
    name: str
    def __init__(self, gateway_id: _Optional[bytes] = ..., name: _Optional[str] = ...) -> None: ...

class DeviceUpdate(_message.Message):
    __slots__ = ("id", "gateway_id", "serial_number", "name", "description", "type_id")
    ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    SERIAL_NUMBER_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TYPE_ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    gateway_id: bytes
    serial_number: str
    name: str
    description: str
    type_id: bytes
    def __init__(self, id: _Optional[bytes] = ..., gateway_id: _Optional[bytes] = ..., serial_number: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., type_id: _Optional[bytes] = ...) -> None: ...

class GatewayUpdate(_message.Message):
    __slots__ = ("id", "serial_number", "name", "description", "type_id")
    ID_FIELD_NUMBER: _ClassVar[int]
    SERIAL_NUMBER_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TYPE_ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    serial_number: str
    name: str
    description: str
    type_id: bytes
    def __init__(self, id: _Optional[bytes] = ..., serial_number: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., type_id: _Optional[bytes] = ...) -> None: ...

class ConfigSchema(_message.Message):
    __slots__ = ("id", "device_id", "name", "config_bytes", "config_type", "category")
    ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CONFIG_BYTES_FIELD_NUMBER: _ClassVar[int]
    CONFIG_TYPE_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    id: int
    device_id: bytes
    name: str
    config_bytes: bytes
    config_type: _common_pb2.ConfigType
    category: str
    def __init__(self, id: _Optional[int] = ..., device_id: _Optional[bytes] = ..., name: _Optional[str] = ..., config_bytes: _Optional[bytes] = ..., config_type: _Optional[_Union[_common_pb2.ConfigType, str]] = ..., category: _Optional[str] = ...) -> None: ...

class ConfigId(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class ConfigUpdate(_message.Message):
    __slots__ = ("id", "name", "config_bytes", "config_type", "category")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CONFIG_BYTES_FIELD_NUMBER: _ClassVar[int]
    CONFIG_TYPE_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    id: int
    name: str
    config_bytes: bytes
    config_type: _common_pb2.ConfigType
    category: str
    def __init__(self, id: _Optional[int] = ..., name: _Optional[str] = ..., config_bytes: _Optional[bytes] = ..., config_type: _Optional[_Union[_common_pb2.ConfigType, str]] = ..., category: _Optional[str] = ...) -> None: ...

class TypeSchema(_message.Message):
    __slots__ = ("id", "name", "description", "models")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    MODELS_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    name: str
    description: str
    models: _containers.RepeatedScalarFieldContainer[bytes]
    def __init__(self, id: _Optional[bytes] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., models: _Optional[_Iterable[bytes]] = ...) -> None: ...

class TypeId(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class TypeName(_message.Message):
    __slots__ = ("name",)
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class TypeUpdate(_message.Message):
    __slots__ = ("id", "name", "description")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    name: str
    description: str
    def __init__(self, id: _Optional[bytes] = ..., name: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class TypeModel(_message.Message):
    __slots__ = ("id", "model_id")
    ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    model_id: bytes
    def __init__(self, id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ...) -> None: ...

class DeviceReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: DeviceSchema
    def __init__(self, result: _Optional[_Union[DeviceSchema, _Mapping]] = ...) -> None: ...

class DeviceListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[DeviceSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[DeviceSchema, _Mapping]]] = ...) -> None: ...

class DeviceCreateResponse(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class DeviceChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GatewayReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: GatewaySchema
    def __init__(self, result: _Optional[_Union[GatewaySchema, _Mapping]] = ...) -> None: ...

class GatewayListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[GatewaySchema]
    def __init__(self, results: _Optional[_Iterable[_Union[GatewaySchema, _Mapping]]] = ...) -> None: ...

class GatewayCreateResponse(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class GatewayChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ConfigReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: ConfigSchema
    def __init__(self, result: _Optional[_Union[ConfigSchema, _Mapping]] = ...) -> None: ...

class ConfigListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[ConfigSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[ConfigSchema, _Mapping]]] = ...) -> None: ...

class ConfigCreateResponse(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class ConfigChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class TypeReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: TypeSchema
    def __init__(self, result: _Optional[_Union[TypeSchema, _Mapping]] = ...) -> None: ...

class TypeListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[TypeSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[TypeSchema, _Mapping]]] = ...) -> None: ...

class TypeCreateResponse(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class TypeChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

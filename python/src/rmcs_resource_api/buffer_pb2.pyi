from rmcs_resource_api import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class BufferSchema(_message.Message):
    __slots__ = ("id", "device_id", "model_id", "timestamp", "data_bytes", "data_type", "status")
    ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DATA_BYTES_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    id: int
    device_id: bytes
    model_id: bytes
    timestamp: int
    data_bytes: bytes
    data_type: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    status: int
    def __init__(self, id: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ..., status: _Optional[int] = ...) -> None: ...

class BufferId(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class BufferTime(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp", "status")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    status: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferSelector(_message.Message):
    __slots__ = ("device_id", "model_id", "status")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    status: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., status: _Optional[int] = ...) -> None: ...

class BuffersSelector(_message.Message):
    __slots__ = ("device_id", "model_id", "status", "number")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    status: int
    number: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., status: _Optional[int] = ..., number: _Optional[int] = ...) -> None: ...

class BufferUpdate(_message.Message):
    __slots__ = ("id", "data_bytes", "data_type", "status")
    ID_FIELD_NUMBER: _ClassVar[int]
    DATA_BYTES_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    id: int
    data_bytes: bytes
    data_type: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    status: int
    def __init__(self, id: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ..., status: _Optional[int] = ...) -> None: ...

class BufferReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: BufferSchema
    def __init__(self, result: _Optional[_Union[BufferSchema, _Mapping]] = ...) -> None: ...

class BufferListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[BufferSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[BufferSchema, _Mapping]]] = ...) -> None: ...

class BufferCreateResponse(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class BufferChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

from rmcs_resource_api import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LogSchema(_message.Message):
    __slots__ = ("timestamp", "device_id", "status", "log_bytes", "log_type")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    LOG_BYTES_FIELD_NUMBER: _ClassVar[int]
    LOG_TYPE_FIELD_NUMBER: _ClassVar[int]
    timestamp: int
    device_id: bytes
    status: int
    log_bytes: bytes
    log_type: _common_pb2.DataType
    def __init__(self, timestamp: _Optional[int] = ..., device_id: _Optional[bytes] = ..., status: _Optional[int] = ..., log_bytes: _Optional[bytes] = ..., log_type: _Optional[_Union[_common_pb2.DataType, str]] = ...) -> None: ...

class LogId(_message.Message):
    __slots__ = ("timestamp", "device_id")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    timestamp: int
    device_id: bytes
    def __init__(self, timestamp: _Optional[int] = ..., device_id: _Optional[bytes] = ...) -> None: ...

class LogTime(_message.Message):
    __slots__ = ("timestamp", "device_id", "status")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    timestamp: int
    device_id: bytes
    status: int
    def __init__(self, timestamp: _Optional[int] = ..., device_id: _Optional[bytes] = ..., status: _Optional[int] = ...) -> None: ...

class LogRange(_message.Message):
    __slots__ = ("begin", "end", "device_id", "status")
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    begin: int
    end: int
    device_id: bytes
    status: int
    def __init__(self, begin: _Optional[int] = ..., end: _Optional[int] = ..., device_id: _Optional[bytes] = ..., status: _Optional[int] = ...) -> None: ...

class LogUpdate(_message.Message):
    __slots__ = ("timestamp", "device_id", "status", "log_bytes", "log_type")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    LOG_BYTES_FIELD_NUMBER: _ClassVar[int]
    LOG_TYPE_FIELD_NUMBER: _ClassVar[int]
    timestamp: int
    device_id: bytes
    status: int
    log_bytes: bytes
    log_type: _common_pb2.DataType
    def __init__(self, timestamp: _Optional[int] = ..., device_id: _Optional[bytes] = ..., status: _Optional[int] = ..., log_bytes: _Optional[bytes] = ..., log_type: _Optional[_Union[_common_pb2.DataType, str]] = ...) -> None: ...

class LogReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: LogSchema
    def __init__(self, result: _Optional[_Union[LogSchema, _Mapping]] = ...) -> None: ...

class LogListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[LogSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[LogSchema, _Mapping]]] = ...) -> None: ...

class LogChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LogSchema(_message.Message):
    __slots__ = ("id", "timestamp", "device_id", "model_id", "tag", "log_bytes", "log_type")
    ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    LOG_BYTES_FIELD_NUMBER: _ClassVar[int]
    LOG_TYPE_FIELD_NUMBER: _ClassVar[int]
    id: int
    timestamp: int
    device_id: bytes
    model_id: bytes
    tag: int
    log_bytes: bytes
    log_type: int
    def __init__(self, id: _Optional[int] = ..., timestamp: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., tag: _Optional[int] = ..., log_bytes: _Optional[bytes] = ..., log_type: _Optional[int] = ...) -> None: ...

class LogId(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class LogTime(_message.Message):
    __slots__ = ("timestamp", "device_id", "model_id", "tag")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    timestamp: int
    device_id: bytes
    model_id: bytes
    tag: int
    def __init__(self, timestamp: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., tag: _Optional[int] = ...) -> None: ...

class LogRange(_message.Message):
    __slots__ = ("begin", "end", "device_id", "model_id", "tag")
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    begin: int
    end: int
    device_id: bytes
    model_id: bytes
    tag: int
    def __init__(self, begin: _Optional[int] = ..., end: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., tag: _Optional[int] = ...) -> None: ...

class LogUpdate(_message.Message):
    __slots__ = ("id", "tag", "log_bytes", "log_type")
    ID_FIELD_NUMBER: _ClassVar[int]
    TAG_FIELD_NUMBER: _ClassVar[int]
    LOG_BYTES_FIELD_NUMBER: _ClassVar[int]
    LOG_TYPE_FIELD_NUMBER: _ClassVar[int]
    id: int
    tag: int
    log_bytes: bytes
    log_type: int
    def __init__(self, id: _Optional[int] = ..., tag: _Optional[int] = ..., log_bytes: _Optional[bytes] = ..., log_type: _Optional[int] = ...) -> None: ...

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

class LogCreateResponse(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class LogChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

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
    data_type: _containers.RepeatedScalarFieldContainer[int]
    status: int
    def __init__(self, id: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[int]] = ..., status: _Optional[int] = ...) -> None: ...

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

class BufferRange(_message.Message):
    __slots__ = ("device_id", "model_id", "begin", "end", "status")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    begin: int
    end: int
    status: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferNumber(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp", "number", "status")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    number: int
    status: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., number: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

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
    __slots__ = ("device_id", "model_id", "status", "number", "offset")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    status: int
    number: int
    offset: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., status: _Optional[int] = ..., number: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class BufferIdsTime(_message.Message):
    __slots__ = ("device_ids", "model_ids", "timestamp", "status")
    DEVICE_IDS_FIELD_NUMBER: _ClassVar[int]
    MODEL_IDS_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_ids: _containers.RepeatedScalarFieldContainer[bytes]
    model_ids: _containers.RepeatedScalarFieldContainer[bytes]
    timestamp: int
    status: int
    def __init__(self, device_ids: _Optional[_Iterable[bytes]] = ..., model_ids: _Optional[_Iterable[bytes]] = ..., timestamp: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferIdsRange(_message.Message):
    __slots__ = ("device_ids", "model_ids", "begin", "end", "status")
    DEVICE_IDS_FIELD_NUMBER: _ClassVar[int]
    MODEL_IDS_FIELD_NUMBER: _ClassVar[int]
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_ids: _containers.RepeatedScalarFieldContainer[bytes]
    model_ids: _containers.RepeatedScalarFieldContainer[bytes]
    begin: int
    end: int
    status: int
    def __init__(self, device_ids: _Optional[_Iterable[bytes]] = ..., model_ids: _Optional[_Iterable[bytes]] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferIdsNumber(_message.Message):
    __slots__ = ("device_ids", "model_ids", "timestamp", "number", "status")
    DEVICE_IDS_FIELD_NUMBER: _ClassVar[int]
    MODEL_IDS_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_ids: _containers.RepeatedScalarFieldContainer[bytes]
    model_ids: _containers.RepeatedScalarFieldContainer[bytes]
    timestamp: int
    number: int
    status: int
    def __init__(self, device_ids: _Optional[_Iterable[bytes]] = ..., model_ids: _Optional[_Iterable[bytes]] = ..., timestamp: _Optional[int] = ..., number: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferIdsSelector(_message.Message):
    __slots__ = ("device_ids", "model_ids", "status")
    DEVICE_IDS_FIELD_NUMBER: _ClassVar[int]
    MODEL_IDS_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_ids: _containers.RepeatedScalarFieldContainer[bytes]
    model_ids: _containers.RepeatedScalarFieldContainer[bytes]
    status: int
    def __init__(self, device_ids: _Optional[_Iterable[bytes]] = ..., model_ids: _Optional[_Iterable[bytes]] = ..., status: _Optional[int] = ...) -> None: ...

class BuffersIdsSelector(_message.Message):
    __slots__ = ("device_ids", "model_ids", "status", "number", "offset")
    DEVICE_IDS_FIELD_NUMBER: _ClassVar[int]
    MODEL_IDS_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    device_ids: _containers.RepeatedScalarFieldContainer[bytes]
    model_ids: _containers.RepeatedScalarFieldContainer[bytes]
    status: int
    number: int
    offset: int
    def __init__(self, device_ids: _Optional[_Iterable[bytes]] = ..., model_ids: _Optional[_Iterable[bytes]] = ..., status: _Optional[int] = ..., number: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class BufferSetTime(_message.Message):
    __slots__ = ("set_id", "timestamp", "status")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    timestamp: int
    status: int
    def __init__(self, set_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferSetRange(_message.Message):
    __slots__ = ("set_id", "begin", "end", "status")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    begin: int
    end: int
    status: int
    def __init__(self, set_id: _Optional[bytes] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferSetNumber(_message.Message):
    __slots__ = ("set_id", "timestamp", "number", "status")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    timestamp: int
    number: int
    status: int
    def __init__(self, set_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., number: _Optional[int] = ..., status: _Optional[int] = ...) -> None: ...

class BufferSetSelector(_message.Message):
    __slots__ = ("set_id", "status")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    status: int
    def __init__(self, set_id: _Optional[bytes] = ..., status: _Optional[int] = ...) -> None: ...

class BuffersSetSelector(_message.Message):
    __slots__ = ("set_id", "status", "number", "offset")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    status: int
    number: int
    offset: int
    def __init__(self, set_id: _Optional[bytes] = ..., status: _Optional[int] = ..., number: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class BufferUpdate(_message.Message):
    __slots__ = ("id", "data_bytes", "data_type", "status")
    ID_FIELD_NUMBER: _ClassVar[int]
    DATA_BYTES_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    id: int
    data_bytes: bytes
    data_type: _containers.RepeatedScalarFieldContainer[int]
    status: int
    def __init__(self, id: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[int]] = ..., status: _Optional[int] = ...) -> None: ...

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

class TimestampReadResponse(_message.Message):
    __slots__ = ("timestamp",)
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    timestamp: int
    def __init__(self, timestamp: _Optional[int] = ...) -> None: ...

class TimestampListResponse(_message.Message):
    __slots__ = ("timestamps",)
    TIMESTAMPS_FIELD_NUMBER: _ClassVar[int]
    timestamps: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, timestamps: _Optional[_Iterable[int]] = ...) -> None: ...

class BufferCountResponse(_message.Message):
    __slots__ = ("count",)
    COUNT_FIELD_NUMBER: _ClassVar[int]
    count: int
    def __init__(self, count: _Optional[int] = ...) -> None: ...

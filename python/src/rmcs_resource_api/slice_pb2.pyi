import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class SliceSchema(_message.Message):
    __slots__ = ["id", "device_id", "model_id", "timestamp_begin", "timestamp_end", "index_begin", "index_end", "name", "description"]
    ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_BEGIN_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_END_FIELD_NUMBER: _ClassVar[int]
    INDEX_BEGIN_FIELD_NUMBER: _ClassVar[int]
    INDEX_END_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    id: int
    device_id: bytes
    model_id: bytes
    timestamp_begin: int
    timestamp_end: int
    index_begin: int
    index_end: int
    name: str
    description: str
    def __init__(self, id: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp_begin: _Optional[int] = ..., timestamp_end: _Optional[int] = ..., index_begin: _Optional[int] = ..., index_end: _Optional[int] = ..., name: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class SliceId(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class SliceName(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class SliceDevice(_message.Message):
    __slots__ = ["device_id"]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    def __init__(self, device_id: _Optional[bytes] = ...) -> None: ...

class SliceModel(_message.Message):
    __slots__ = ["model_id"]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    model_id: bytes
    def __init__(self, model_id: _Optional[bytes] = ...) -> None: ...

class SliceDeviceModel(_message.Message):
    __slots__ = ["device_id", "model_id"]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ...) -> None: ...

class SliceUpdate(_message.Message):
    __slots__ = ["id", "timestamp_begin", "timestamp_end", "index_begin", "index_end", "name", "description"]
    ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_BEGIN_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_END_FIELD_NUMBER: _ClassVar[int]
    INDEX_BEGIN_FIELD_NUMBER: _ClassVar[int]
    INDEX_END_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    id: int
    timestamp_begin: int
    timestamp_end: int
    index_begin: int
    index_end: int
    name: str
    description: str
    def __init__(self, id: _Optional[int] = ..., timestamp_begin: _Optional[int] = ..., timestamp_end: _Optional[int] = ..., index_begin: _Optional[int] = ..., index_end: _Optional[int] = ..., name: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class SliceReadResponse(_message.Message):
    __slots__ = ["result"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: SliceSchema
    def __init__(self, result: _Optional[_Union[SliceSchema, _Mapping]] = ...) -> None: ...

class SliceListResponse(_message.Message):
    __slots__ = ["results"]
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[SliceSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[SliceSchema, _Mapping]]] = ...) -> None: ...

class SliceCreateResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class SliceChangeResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

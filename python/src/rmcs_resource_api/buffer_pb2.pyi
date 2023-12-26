from rmcs_resource_api import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class BufferStatus(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    DEFAULT: _ClassVar[BufferStatus]
    ERROR: _ClassVar[BufferStatus]
    DELETE: _ClassVar[BufferStatus]
    HOLD: _ClassVar[BufferStatus]
    SEND_UPLINK: _ClassVar[BufferStatus]
    SEND_DOWNLINK: _ClassVar[BufferStatus]
    TRANSFER_LOCAL: _ClassVar[BufferStatus]
    TRANSFER_GATEWAY: _ClassVar[BufferStatus]
    TRANSFER_SERVER: _ClassVar[BufferStatus]
    BACKUP: _ClassVar[BufferStatus]
    RESTORE: _ClassVar[BufferStatus]
    ANALYSIS_1: _ClassVar[BufferStatus]
    ANALYSIS_2: _ClassVar[BufferStatus]
    ANALYSIS_3: _ClassVar[BufferStatus]
    ANALYSIS_4: _ClassVar[BufferStatus]
    ANALYSIS_5: _ClassVar[BufferStatus]
    ANALYSIS_6: _ClassVar[BufferStatus]
    ANALYSIS_7: _ClassVar[BufferStatus]
    ANALYSIS_8: _ClassVar[BufferStatus]
    ANALYSIS_9: _ClassVar[BufferStatus]
    ANALYSIS_10: _ClassVar[BufferStatus]
DEFAULT: BufferStatus
ERROR: BufferStatus
DELETE: BufferStatus
HOLD: BufferStatus
SEND_UPLINK: BufferStatus
SEND_DOWNLINK: BufferStatus
TRANSFER_LOCAL: BufferStatus
TRANSFER_GATEWAY: BufferStatus
TRANSFER_SERVER: BufferStatus
BACKUP: BufferStatus
RESTORE: BufferStatus
ANALYSIS_1: BufferStatus
ANALYSIS_2: BufferStatus
ANALYSIS_3: BufferStatus
ANALYSIS_4: BufferStatus
ANALYSIS_5: BufferStatus
ANALYSIS_6: BufferStatus
ANALYSIS_7: BufferStatus
ANALYSIS_8: BufferStatus
ANALYSIS_9: BufferStatus
ANALYSIS_10: BufferStatus

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
    status: BufferStatus
    def __init__(self, id: _Optional[int] = ..., device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ..., status: _Optional[_Union[BufferStatus, str]] = ...) -> None: ...

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
    status: BufferStatus
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., status: _Optional[_Union[BufferStatus, str]] = ...) -> None: ...

class BufferSelector(_message.Message):
    __slots__ = ("device_id", "model_id", "status")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    status: BufferStatus
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., status: _Optional[_Union[BufferStatus, str]] = ...) -> None: ...

class BuffersSelector(_message.Message):
    __slots__ = ("device_id", "model_id", "status", "number")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    status: BufferStatus
    number: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., status: _Optional[_Union[BufferStatus, str]] = ..., number: _Optional[int] = ...) -> None: ...

class BufferUpdate(_message.Message):
    __slots__ = ("id", "data_bytes", "data_type", "status")
    ID_FIELD_NUMBER: _ClassVar[int]
    DATA_BYTES_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    id: int
    data_bytes: bytes
    data_type: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    status: BufferStatus
    def __init__(self, id: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ..., status: _Optional[_Union[BufferStatus, str]] = ...) -> None: ...

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

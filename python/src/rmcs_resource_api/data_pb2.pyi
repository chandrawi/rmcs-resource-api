from rmcs_resource_api import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class DataSchema(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp", "data_bytes", "data_type")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DATA_BYTES_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    data_bytes: bytes
    data_type: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ...) -> None: ...

class DataId(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ...) -> None: ...

class DataTime(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ...) -> None: ...

class DataRange(_message.Message):
    __slots__ = ("device_id", "model_id", "begin", "end")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    begin: int
    end: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...

class DataNumber(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp", "number")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    number: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., number: _Optional[int] = ...) -> None: ...

class DataSetSchema(_message.Message):
    __slots__ = ("set_id", "timestamp", "data_bytes", "data_type")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    DATA_BYTES_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    timestamp: int
    data_bytes: bytes
    data_type: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    def __init__(self, set_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., data_bytes: _Optional[bytes] = ..., data_type: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ...) -> None: ...

class DataSetId(_message.Message):
    __slots__ = ("set_id", "timestamp")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    timestamp: int
    def __init__(self, set_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ...) -> None: ...

class DataSetTime(_message.Message):
    __slots__ = ("set_id", "timestamp")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    timestamp: int
    def __init__(self, set_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ...) -> None: ...

class DataSetRange(_message.Message):
    __slots__ = ("set_id", "begin", "end")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    begin: int
    end: int
    def __init__(self, set_id: _Optional[bytes] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...

class DataSetNumber(_message.Message):
    __slots__ = ("set_id", "timestamp", "number")
    SET_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    NUMBER_FIELD_NUMBER: _ClassVar[int]
    set_id: bytes
    timestamp: int
    number: int
    def __init__(self, set_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., number: _Optional[int] = ...) -> None: ...

class DataCount(_message.Message):
    __slots__ = ("device_id", "model_id", "timestamp", "begin", "end")
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    BEGIN_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    device_id: bytes
    model_id: bytes
    timestamp: int
    begin: int
    end: int
    def __init__(self, device_id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ..., timestamp: _Optional[int] = ..., begin: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...

class DataReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: DataSchema
    def __init__(self, result: _Optional[_Union[DataSchema, _Mapping]] = ...) -> None: ...

class DataListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[DataSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[DataSchema, _Mapping]]] = ...) -> None: ...

class DataChangeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DataCountResponse(_message.Message):
    __slots__ = ("count",)
    COUNT_FIELD_NUMBER: _ClassVar[int]
    count: int
    def __init__(self, count: _Optional[int] = ...) -> None: ...

class DataSetReadResponse(_message.Message):
    __slots__ = ("result",)
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: DataSetSchema
    def __init__(self, result: _Optional[_Union[DataSetSchema, _Mapping]] = ...) -> None: ...

class DataSetListResponse(_message.Message):
    __slots__ = ("results",)
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[DataSetSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[DataSetSchema, _Mapping]]] = ...) -> None: ...

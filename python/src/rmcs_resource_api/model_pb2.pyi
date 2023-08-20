import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ModelSchema(_message.Message):
    __slots__ = ["id", "indexing", "category", "name", "description", "types", "configs"]
    ID_FIELD_NUMBER: _ClassVar[int]
    INDEXING_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TYPES_FIELD_NUMBER: _ClassVar[int]
    CONFIGS_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    indexing: _common_pb2.DataIndexing
    category: str
    name: str
    description: str
    types: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    configs: _containers.RepeatedCompositeFieldContainer[ConfigSchemaVec]
    def __init__(self, id: _Optional[bytes] = ..., indexing: _Optional[_Union[_common_pb2.DataIndexing, str]] = ..., category: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., types: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ..., configs: _Optional[_Iterable[_Union[ConfigSchemaVec, _Mapping]]] = ...) -> None: ...

class ConfigSchemaVec(_message.Message):
    __slots__ = ["configs"]
    CONFIGS_FIELD_NUMBER: _ClassVar[int]
    configs: _containers.RepeatedCompositeFieldContainer[ConfigSchema]
    def __init__(self, configs: _Optional[_Iterable[_Union[ConfigSchema, _Mapping]]] = ...) -> None: ...

class ModelId(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class ModelName(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class ModelCategory(_message.Message):
    __slots__ = ["category"]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    category: str
    def __init__(self, category: _Optional[str] = ...) -> None: ...

class ModelNameCategory(_message.Message):
    __slots__ = ["name", "category"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    name: str
    category: str
    def __init__(self, name: _Optional[str] = ..., category: _Optional[str] = ...) -> None: ...

class ModelUpdate(_message.Message):
    __slots__ = ["id", "indexing", "category", "name", "description"]
    ID_FIELD_NUMBER: _ClassVar[int]
    INDEXING_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    indexing: _common_pb2.DataIndexing
    category: str
    name: str
    description: str
    def __init__(self, id: _Optional[bytes] = ..., indexing: _Optional[_Union[_common_pb2.DataIndexing, str]] = ..., category: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class ModelTypes(_message.Message):
    __slots__ = ["id", "types"]
    ID_FIELD_NUMBER: _ClassVar[int]
    TYPES_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    types: _containers.RepeatedScalarFieldContainer[_common_pb2.DataType]
    def __init__(self, id: _Optional[bytes] = ..., types: _Optional[_Iterable[_Union[_common_pb2.DataType, str]]] = ...) -> None: ...

class ConfigSchema(_message.Message):
    __slots__ = ["id", "model_id", "index", "name", "config_bytes", "config_type", "category"]
    ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    INDEX_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CONFIG_BYTES_FIELD_NUMBER: _ClassVar[int]
    CONFIG_TYPE_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    id: int
    model_id: bytes
    index: int
    name: str
    config_bytes: bytes
    config_type: _common_pb2.ConfigType
    category: str
    def __init__(self, id: _Optional[int] = ..., model_id: _Optional[bytes] = ..., index: _Optional[int] = ..., name: _Optional[str] = ..., config_bytes: _Optional[bytes] = ..., config_type: _Optional[_Union[_common_pb2.ConfigType, str]] = ..., category: _Optional[str] = ...) -> None: ...

class ConfigId(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class ConfigUpdate(_message.Message):
    __slots__ = ["id", "name", "config_bytes", "config_type", "category"]
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

class ModelReadResponse(_message.Message):
    __slots__ = ["result"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: ModelSchema
    def __init__(self, result: _Optional[_Union[ModelSchema, _Mapping]] = ...) -> None: ...

class ModelListResponse(_message.Message):
    __slots__ = ["results"]
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[ModelSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[ModelSchema, _Mapping]]] = ...) -> None: ...

class ModelCreateResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class ModelChangeResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ConfigReadResponse(_message.Message):
    __slots__ = ["result"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: ConfigSchema
    def __init__(self, result: _Optional[_Union[ConfigSchema, _Mapping]] = ...) -> None: ...

class ConfigListResponse(_message.Message):
    __slots__ = ["results"]
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[ConfigSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[ConfigSchema, _Mapping]]] = ...) -> None: ...

class ConfigCreateResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class ConfigChangeResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

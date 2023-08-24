from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GroupModelSchema(_message.Message):
    __slots__ = ["id", "name", "category", "description", "models"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    MODELS_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    name: str
    category: str
    description: str
    models: _containers.RepeatedScalarFieldContainer[bytes]
    def __init__(self, id: _Optional[bytes] = ..., name: _Optional[str] = ..., category: _Optional[str] = ..., description: _Optional[str] = ..., models: _Optional[_Iterable[bytes]] = ...) -> None: ...

class GroupDeviceSchema(_message.Message):
    __slots__ = ["id", "name", "category", "description", "devices"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DEVICES_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    name: str
    category: str
    description: str
    devices: _containers.RepeatedScalarFieldContainer[bytes]
    def __init__(self, id: _Optional[bytes] = ..., name: _Optional[str] = ..., category: _Optional[str] = ..., description: _Optional[str] = ..., devices: _Optional[_Iterable[bytes]] = ...) -> None: ...

class GroupId(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class GroupName(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class GroupCategory(_message.Message):
    __slots__ = ["category"]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    category: str
    def __init__(self, category: _Optional[str] = ...) -> None: ...

class GroupNameCategory(_message.Message):
    __slots__ = ["name", "category"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    name: str
    category: str
    def __init__(self, name: _Optional[str] = ..., category: _Optional[str] = ...) -> None: ...

class GroupUpdate(_message.Message):
    __slots__ = ["id", "name", "category", "description"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    name: str
    category: str
    description: str
    def __init__(self, id: _Optional[bytes] = ..., name: _Optional[str] = ..., category: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class GroupModel(_message.Message):
    __slots__ = ["id", "model_id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    model_id: bytes
    def __init__(self, id: _Optional[bytes] = ..., model_id: _Optional[bytes] = ...) -> None: ...

class GroupDevice(_message.Message):
    __slots__ = ["id", "device_id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    device_id: bytes
    def __init__(self, id: _Optional[bytes] = ..., device_id: _Optional[bytes] = ...) -> None: ...

class GroupModelReadResponse(_message.Message):
    __slots__ = ["result"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: GroupModelSchema
    def __init__(self, result: _Optional[_Union[GroupModelSchema, _Mapping]] = ...) -> None: ...

class GroupModelListResponse(_message.Message):
    __slots__ = ["results"]
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[GroupModelSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[GroupModelSchema, _Mapping]]] = ...) -> None: ...

class GroupDeviceReadResponse(_message.Message):
    __slots__ = ["result"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: GroupDeviceSchema
    def __init__(self, result: _Optional[_Union[GroupDeviceSchema, _Mapping]] = ...) -> None: ...

class GroupDeviceListResponse(_message.Message):
    __slots__ = ["results"]
    RESULTS_FIELD_NUMBER: _ClassVar[int]
    results: _containers.RepeatedCompositeFieldContainer[GroupDeviceSchema]
    def __init__(self, results: _Optional[_Iterable[_Union[GroupDeviceSchema, _Mapping]]] = ...) -> None: ...

class GroupCreateResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: bytes
    def __init__(self, id: _Optional[bytes] = ...) -> None: ...

class GroupChangeResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from typing import ClassVar as _ClassVar

DESCRIPTOR: _descriptor.FileDescriptor

class DataType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    NULLD: _ClassVar[DataType]
    I8: _ClassVar[DataType]
    I16: _ClassVar[DataType]
    I32: _ClassVar[DataType]
    I64: _ClassVar[DataType]
    I128: _ClassVar[DataType]
    U8: _ClassVar[DataType]
    U16: _ClassVar[DataType]
    U32: _ClassVar[DataType]
    U64: _ClassVar[DataType]
    U128: _ClassVar[DataType]
    F32: _ClassVar[DataType]
    F64: _ClassVar[DataType]
    BOOL: _ClassVar[DataType]
    CHAR: _ClassVar[DataType]

class ConfigType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    NULLC: _ClassVar[ConfigType]
    INT: _ClassVar[ConfigType]
    FLOAT: _ClassVar[ConfigType]
    STR: _ClassVar[ConfigType]
NULLD: DataType
I8: DataType
I16: DataType
I32: DataType
I64: DataType
I128: DataType
U8: DataType
U16: DataType
U32: DataType
U64: DataType
U128: DataType
F32: DataType
F64: DataType
BOOL: DataType
CHAR: DataType
NULLC: ConfigType
INT: ConfigType
FLOAT: ConfigType
STR: ConfigType

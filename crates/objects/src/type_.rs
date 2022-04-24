// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Byte, // 1 Byte 
    Integer, // 4 Bytes like an `i32`
    BigInteger, // 8 bytes like an `i64`

    Str, // len(str) * sizeof(Byte)
    NotNative(String),
}

pub fn string_to_type(type_as_string: String) -> Type {
    match type_as_string.as_str() {
        "byte" => Type::Byte,
        "int" => Type::Integer,
        "bigint" => Type::BigInteger,
        "str" => Type::Str,
        _ => Type::NotNative(type_as_string),
    }
}

pub fn type_to_asm(type_: &Type) -> String {
    match type_ {
        Type::Byte => "db",
        Type::Integer => "dd",
        Type::BigInteger => "dq",
        Type::Str => "db",
        Type::NotNative(ref type_as_string) => &*type_as_string,
    }
    .to_string()
}

pub fn type_size_to_asm(type_: &Type) -> String {
    match type_ {
        Type::Byte => "byte",
        Type::Integer => "dword",
        Type::BigInteger => "qword",
        Type::Str => "byte",
        Type::NotNative(ref _type_as_string) => todo!()
    }
    .to_string()
}

pub fn type_size_to_usize(type_: &Type) -> usize {
    match type_ {
        Type::Byte => 1,
        Type::Integer => 4,
        Type::BigInteger => 8,
        Type::Str => 1,
        Type::NotNative(ref _type_as_string) => todo!()
    }
}
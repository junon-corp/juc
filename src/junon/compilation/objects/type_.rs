// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Integer,
    UnsignedInteger,
    Float,
    Str,

    NotNative(String),
}

pub fn string_to_type(type_as_string: String) -> Type {
    match type_as_string.as_str() {
        "int" => Type::Integer,
        "uint" => Type::UnsignedInteger,
        "float" => Type::Float,
        "str" => Type::Str,
        _ => Type::NotNative(type_as_string),
    }
}

pub fn type_to_asm(type_: Type) -> String {
    match type_ {
        Type::Integer => "dw",
        Type::UnsignedInteger => "dw",
        Type::Float => "dd",
        Type::Str => "db",
        Type::NotNative(ref type_as_string) => &*type_as_string,
    }
    .to_string()
}

pub fn type_size_to_asm(type_: Type) -> String {
    match type_ {
        Type::Integer => "word",
        Type::UnsignedInteger => "word",
        Type::Float => "dword",
        Type::Str => "byte",
        Type::NotNative(ref _type_as_string) => todo!()
    }
    .to_string()
}

pub fn type_size_to_usize(type_: &Type) -> usize {
    match type_ {
        Type::Integer => 2,
        Type::UnsignedInteger => 2,
        Type::Float => 4,
        Type::Str => 1,
        Type::NotNative(ref _type_as_string) => todo!()
    }
}
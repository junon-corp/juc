// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

#[derive(Debug, Clone, Eq, PartialEq)]
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
        Type::NotNative(ref type_as_string) => &*type_as_string
    }.to_string()
}

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

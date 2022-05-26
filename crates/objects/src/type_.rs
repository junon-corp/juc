// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use x64asm::ddirective;
use x64asm::ddirective::DefineDirective::*;
use x64asm::operand::Operand;

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Byte,       // 1 Byte
    Integer,    // 4 Bytes like an `i32`
    BigInteger, // 8 bytes like an `i64`

    StaticArray(Box<Self>),     // sizeof(Type)
    Array(Box<Self>, usize),    // sizeof(Type) * len
}

impl Type {
    pub fn from_string(type_as_string: String) -> Self {
        match type_as_string.as_str() {
            "byte" => Type::Byte,
            "int" => Type::Integer,
            "bigint" => Type::BigInteger,
            _ => panic!(),
        }
    }

    pub fn array_from_string(type_as_string: String, len: usize) -> Self {
        let type_ = Self::from_string(type_as_string);
        Type::Array(Box::new(type_), len)
    }

    pub fn new_array(type_: Type, len: usize) -> Self {
        Type::Array(Box::new(type_), len)
    }

    pub fn static_array_from_string(type_as_string: String) -> Self {
        let type_ = Self::from_string(type_as_string);
        Type::StaticArray(Box::new(type_))
    }

    pub fn to_asm_operand(&self) -> Operand {
        // The `ddirective!()` macro create an operand object
        match *self {
            Self::Byte => ddirective!(Db),
            Self::Integer => ddirective!(Dd),
            Self::BigInteger => ddirective!(Dq),
            Self::Array(ref type_, _) 
                | Self::StaticArray(ref type_) => (*type_).to_asm_operand()
        }
    }

    pub fn to_usize(&self) -> usize {
        match *self {
            Self::Byte => 1,
            Self::Integer => 4,
            Self::BigInteger => 8,
            Self::Array(ref type_, len) => (*type_).to_usize() * len,
            Self::StaticArray(ref type_) => (*type_).to_usize(),
        }
    }
}

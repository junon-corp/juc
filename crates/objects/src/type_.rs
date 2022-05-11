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

    Str, // len(str) * sizeof(Byte)
    NotNative(String),
}

impl Type {
    pub fn from_string(type_as_string: String) -> Self {
        match type_as_string.as_str() {
            "byte" => Type::Byte,
            "int" => Type::Integer,
            "bigint" => Type::BigInteger,
            "str" => Type::Str,
            _ => Type::NotNative(type_as_string),
        }
    }

    pub fn to_asm_operand(&self) -> Operand {
        // The `ddirective!()` macro create an operand object
        ddirective!(match *self {
            Type::Byte => Db,
            Type::Integer => Dd,
            Type::BigInteger => Dq,
            Type::Str => Db,
            Type::NotNative(ref _type_as_string) => todo!(),
        })
    }

    pub fn to_usize(&self) -> usize {
        match *self {
            Type::Byte => 1,
            Type::Integer => 4,
            Type::BigInteger => 8,
            Type::Str => 1,
            Type::NotNative(ref _type_as_string) => todo!(),
        }
    }
}

// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

pub enum Type {
    Integer,
    UnsignedInteger,
    BigInteger,
    BigUnsignedInteger,

    Float,
    BigFloat,
    
    Pointer(Box<Type>),
    // Reference,
}

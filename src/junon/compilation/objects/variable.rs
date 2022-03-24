// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::compilation::objects::type_::Type;

/// Structure used to create a variable \
/// Because it's not an interpreter, this is stored following the variable, this
/// structure should only be used by defining a variable in ASM
pub struct Variable {
    id: String,
    type_: Type,
    init_value: String,
}

impl Variable {
    pub fn new(id: String, type_: Type, init_value: String) -> Self {
        Self {
            id,
            type_,
            init_value,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn type_(&self) -> &Type {
        &self.type_
    }
    pub fn init_value(&self) -> &String {
        &self.init_value
    }
}

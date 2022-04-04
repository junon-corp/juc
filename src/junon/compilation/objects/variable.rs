// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::compilation::objects::type_::Type;

/// Structure used to create a variable \
/// Because it's not an interpreter, this is stored following the variable, this
/// structure should only be used by defining a variable in ASM
#[derive(PartialEq, Eq, Hash)] // needed for `Dict` objects
#[derive(Clone)]
pub struct Variable {
    id: String,
    type_: Type,
    current_value: String,
    stack_pos: usize,
}

impl Variable {
    pub fn new(
        id: String, 
        type_: Type, 
        current_value: String, 
        stack_pos: usize
    ) -> Self {
        
        Self {
            id,
            type_,
            current_value,
            stack_pos,
        }
    }

    pub fn static_(id: String, type_: Type, value: String) -> Self {
        Self {
            id,
            type_,
            current_value: value,
            stack_pos: 0
        }
    }

    pub fn set_current_value(&mut self, current_value: String) {
        self.current_value = current_value;
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn type_(&self) -> &Type {
        &self.type_
    }
    pub fn current_value(&self) -> &String {
        &self.current_value
    }
    pub fn stack_pos(&self) -> &usize {
        &self.stack_pos
    }
}

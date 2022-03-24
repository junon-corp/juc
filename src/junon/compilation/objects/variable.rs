// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::{
    compilation::{
        objects::{
            type_::Type,
        },
    },
};

pub struct Variable {
    id: String,
    type_: Type,
}

impl Variable {
    pub fn new(id: String, type_: Type) -> Self {
        Self {
            id,
            type_
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn type_(&self) -> &Type {
        &self.type_
    }
}

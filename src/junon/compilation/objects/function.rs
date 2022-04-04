// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::compilation::objects::params::Params;

/// Structure used to create a function \
/// Because it's not an interpreter, this is stored following the function, this
/// structure should only be used by defining a function in ASM
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub struct Function {
    id: String,
    params: Params,
    return_type: String,
}

impl Function {
    pub fn new(id: String, params: Params, return_type: String) -> Self {
        Self {
            id,
            params,
            return_type,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn params(&self) -> &Params {
        &self.params
    }
    pub fn return_type(&self) -> &String {
        &self.return_type
    }
}

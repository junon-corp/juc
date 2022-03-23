// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::{
    compilation::{
        objects::{
            params::Params,
        },
    },
};

pub struct Function {
    id: String,
    params: Params,
    return_type: String,
}

impl Function {
    pub fn new(
        id: String, 
        params: Params, 
        return_type: String
    ) -> Self {
        Self {
            id,
            params,
            return_type
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

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
    parameters: Params,
    return_type: String,
}

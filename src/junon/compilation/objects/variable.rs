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

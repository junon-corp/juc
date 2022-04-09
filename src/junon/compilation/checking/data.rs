// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::{
    compilation::parsing::{
        parser::Parser,
        tokens::Token,
    },
    logger::*,
};

/// Information for all checker structures
pub struct CheckerData {
    pub source: String,
    pub parsed: Vec<Vec<Token>>,
    pub logger: Logger,
    pub line_i: usize,
    pub token_i: usize,
}

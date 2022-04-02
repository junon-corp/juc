// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::fs::File;

use crate::junon::compilation::{
    objects::variable::Variable,
    parsing::{
        parser::Parser,
        tokens::Token,
    },
};

/// Important information given to the compiler
pub struct CompilerData {
    pub sources: Vec<String>,
    pub options: Dict<String, String>,
    pub is_library: bool,
    pub stream: Option<File>,
    pub parser: Option<Parser>,

    pub variable_stack: Dict<String, Variable>, // id, position in stack
    pub i_variable_stack: usize,

    pub current_line: Vec<Token>,
    pub current_token: Token,

    pub current_scope: String, // example: "main::f" -> mod "main", function "f"
}

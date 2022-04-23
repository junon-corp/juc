// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::fs::File;

use jup::{
    parser::Parser, 
    tokens::Token,
};

use crate::junon::compilation::{
    objects::{
        variable::Variable,
    },
    scope::Scope,
};

/// Important information given to the compiler
pub struct CompilerData {
    pub is_library: bool,

    pub sources: Vec<String>,
    pub options: Dict<String, String>,
    
    pub stream: Option<File>,
    pub parser: Option<Parser>,

    pub current_scope: Scope,
    pub current_line: Vec<Token>,
    pub current_token: Token,
    //                        id
    pub variable_stack: Dict<String, Variable>,
    pub i_variable_stack: usize,
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use jup::lang::{
    elements::{ Element, variable::Variable },
    tokens::Token,
};
use x64asm::formatter::Formatter;
use crate::scope::Scope;

/// Important information given to the compiler
pub struct CompilerData {
    pub is_library: bool,

    pub sources: Vec<String>,
    pub options: Dict<String, String>,

    pub asm_formatter: Formatter,

    pub current_source: String,
    pub current_parsed: Vec<Element>,
    pub current_scope: Scope,

    //                        id
    pub variable_stack: Dict<String, Variable>,
    pub i_variable_stack: usize,
}

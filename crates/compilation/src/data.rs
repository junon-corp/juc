// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::fs::File;

use jup::{
    parser::Parser, 
    lang::tokens::Token,
};

use x64asm::formatter::Formatter;

use objects::{
    variable::Variable,
};

use crate::scope::Scope;

/// Important information given to the compiler
pub struct CompilerData {
    pub is_library: bool,

    pub sources: Vec<String>,
    pub options: Dict<String, String>,
    
    pub asm_formatter: Formatter,

    pub current_source: String,
    pub current_scope: Scope,
    pub current_token: Token,
    /// Current source's content but parsed
    pub current_parsed: Vec<Token>,

    //                        id
    pub variable_stack: Dict<String, Variable>,
    pub i_variable_stack: usize,
}

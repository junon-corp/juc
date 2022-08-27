// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

//! All these structures will be used for the compilers
//! These functions should be implemented :
//! ```
//! fn data(&mut self) -> &mut CompilerData;
//! fn tools(&mut self) -> &mut CompilerTools;
//! fn code_data(&mut self) -> &mut CompilerCodeData;
//! fn stacks_data(&mut self) -> &mut CompilerStacksData;
//! ```

use std::collections::HashMap as Dict;

use jup::lang::{
    tokens::Token,
    elements::{ 
        Element,
        variable::Variable 
    }
};
use x64asm::formatter::Formatter;
use strings::manager::StringsManager;

use crate::scope::Scope;

/// Some useful variables for the compiler
pub struct CompilerData {
    pub is_library: bool,
    
    pub sources: Vec<String>,
    pub options: Dict<String, String>,
}

/// Some tools used by the compilers
pub struct CompilerTools {
    pub strings_manager: StringsManager,
    pub asm_formatter: Formatter,
}

/// Some useful variables for code walking
pub struct CompilerCodeData {
    pub current_source: String,
    pub current_parsed: Vec<Element>,
    pub scope: Scope,
    
    pub next_element: Element,
    /// Should the next element be skipped ?
    pub is_skip_next: bool,

    pub is_condition: bool,
    pub n_condition: i32,

    pub is_loop: bool,
    pub n_loop: i32,

    pub previous_operator: Token,
}

/// Some useful variables for the Assembly stacks
pub struct CompilerStacksData {
    /// First one is the variable's id
    pub variable_stack: Dict<String, Variable>,
    pub i_variable_stack: usize,

    pub i_parameter_stack: usize,
}
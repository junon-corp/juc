// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::{
    compilation::{
        objects::{
            function::Function,
            variable::Variable,
        },
        base,
        data::CompilerData,
    },
};

pub struct LinuxCompiler {
    data: CompilerData
}

impl LinuxCompiler {
    pub fn new(data: CompilerData) -> Self {
        Self {
            data,
        }
    }
}

/// SEE Functions' documentation from `base::Compiler` because they are not 
/// written here a new time
impl base::Compiler for LinuxCompiler {
    fn init(&mut self) {

    }

    fn run(&mut self) {

    }
    
    fn link(&mut self) {

    }

    fn finish(&mut self) {

    }

    fn data(&mut self) -> &mut CompilerData {
        &mut self.data
    }

    // --- ASM code generators

    fn add_variable(&mut self, variable: Variable) {

    }
    
    fn add_function(&mut self, function: Function) {

    }

    // fn add_structure(&mut self, structure: Structure) {}
}

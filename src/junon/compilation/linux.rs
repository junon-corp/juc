// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::fs::File;
use std::io::Write;

use crate::junon::{
    compilation::{
        objects::{
            function::Function,
            variable::Variable,
        },
        base,
        data::CompilerData,
        defaults::*, defaults::linux_defaults::*,
    },
    args::Args,
    platform,
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
        if self.data().is_library { return; }
    
        let to_write: String = format!(
            "section .text\n\
                \tglobal {}\n\
            extern {}\n\
            {}:\n\
                \tcall {}",

            START_FUNCTION, ENTRY_POINT, START_FUNCTION, ENTRY_POINT,
        );

        File::create(format!("{}/{}", BUILD_FOLDER, START_FILE)).unwrap()
            .write_all(to_write.as_bytes())
            .unwrap(); // already checked before

        platform::exec(ASSEMBLER.to_string(), 
            /* arguments */ &[
                format!("{}/{}", BUILD_FOLDER, START_FILE), 
                "-f".to_string(), "elf64".to_string(),
                "-o".to_string(), format!("{}/{}.o", BUILD_FOLDER, START_FILE)
            ],
        );
    }
    
    fn link(&mut self) {
        let mut bin_filename: String = OUTPUT_FILE.to_string();
        Args::when_flag('o', &self.data().options, | bin_filename_: String | {
            bin_filename = bin_filename_;
        });

        let mut args = vec!(
            "-o".to_string(), bin_filename.to_string(),
        );
        if self.data().is_library {
            args.push("-shared".to_string());
        } else {
            // When it's a library, the start file is not created
            args.push(format!("{}/{}.o", BUILD_FOLDER, START_FILE));
        }

        for source in &self.data().sources {
            args.push(format!("{}/{}.o", BUILD_FOLDER, source));
        }

        platform::exec(LINKER.to_string(), &args);
    }

    fn finish(&mut self) {

    }

    fn finish_one(&mut self, source: &String) {
        platform::exec(ASSEMBLER.to_string(), 
            // Arguments 
            &[
                format!("{}/{}.asm", BUILD_FOLDER, source), 
                
                // Compiling to elf64 object file type
                "-f".to_string(), "elf64".to_string(),

                // The output is the same name than the source file but with 
                // an ".o" extension
                "-o".to_string(), format!("{}/{}.o", BUILD_FOLDER, source)
            ],
        );
    }

    fn data(&mut self) -> &mut CompilerData {
        &mut self.data
    }

    // --- ASM code generators

    fn add_variable(&mut self, variable: Variable) {

    }
    
    fn add_function(&mut self, function: Function) {
        match &mut self.data().stream {
            Some(stream) => {
                write!(stream,
                    "section .text\n\
                    \tglobal {}\n\
                    {}:\n\
                    \tret\n\
                    ",
                    function.id(), function.id()
                );
            },
            None => panic!(), // never happens
        }
    }

    // fn add_structure(&mut self, structure: Structure) {}
}

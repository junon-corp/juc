// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::{
    fs,
    path::Path,
};

use jup::{
    lang::{
        elements::{
            Element,
            function::Function, 
            operation::Operation, 
            type_::Type, 
            variable::Variable
        },
        tokens::Token,
    },
    parser::Parser,
    tokenizer::Tokenizer,
};
use x64asm::{
    formatter::Formatter, 
    instruction as i, 
    instruction::Instruction, 
    label, 
    mnemonic::Mnemonic,
    mnemonic::Mnemonic::*,
    operand::{Op, Operand},
    reg, 
    register::Register,
    register::Register::*,
    section, 
    section::Section::*,
};

use args::Args;
use platform;

use crate::{
    compilers::base::Compiler,
    data::{
        CompilerData,
        CompilerTools,
        CompilerCodeData,
        CompilerStacksData,
    },
    defaults,
    defaults::linux_defaults,
};

/// Compiles for Linux platforms, using the "nasm" assembler
///
/// Uses intel syntax and x64 Assembly
pub struct LinuxCompiler {
    // Data objects as it's required
    data: CompilerData,
    tools: CompilerTools,
    code_data: CompilerCodeData,
    stacks_data: CompilerStacksData,

    section_data: Vec<Instruction>,
    output_dir: String,
}

impl LinuxCompiler {
    pub fn new(all_data: (CompilerData, CompilerTools, CompilerCodeData, CompilerStacksData)) -> Self {
        Self {
            data: all_data.0,
            tools: all_data.1,
            code_data: all_data.2,
            stacks_data: all_data.3,

            section_data: vec![],
            output_dir: String::new()
        }
    }
}


/// See some functions' documentations on the `Compiler` page because they are 
/// not written here already
impl Compiler for LinuxCompiler {
    fn init(&mut self) {
        let mut output_dir = String::new();

        Args::when_flag('o', &self.data().options, |args_output_path: String| {
            let output_dir_path = Path::new(&args_output_path)
                .parent()
                .unwrap();

            output_dir = output_dir_path.to_str()
                .unwrap()
                .to_string();
                
            fs::create_dir_all(output_dir_path).unwrap();
        });

        self.output_dir = output_dir;
    }

    fn terminate(&mut self) {}

    /// Creates an Assembly file for the source file.
    ///
    /// Parses file's content and sets `self.code_data().current_parsed`.  
    fn init_file(&mut self, source_path: &String) {
        // Here we are talking about the "output file" as the this file's 
        // output, not the output generated after linking

        let output_file_path: String = format!(
            "{}/{}.asm",
            self.output_dir,
            source_path
        );
        self.code_data().current_source = output_file_path.clone();
        
        // Parses the tokens to something that could be used by the compiler 
        // (elements) 
        let mut parser = Parser::new({
            // Reads the source file content and transforms it into tokens
            let mut tokenizer = Tokenizer::from_path(Path::new(source_path))
                .unwrap();
            tokenizer.run();
            tokenizer.tokenized().clone()
        });
        parser.run();

        self.code_data().current_parsed = parser.parsed().clone();
    }

    /// Terminates to write some Assembly code if needed (data sections)
    ///
    /// Assembles Assembly code to an object file to be linked  
    fn terminate_file(&mut self, source_path: &String) {
        // Adds all items from the data section
        if !self.section_data.is_empty() {
            self.tools().asm_formatter.add_instruction(
                i!(section!(Data))
            );

            let mut section_data = self.section_data.clone();
            self.tools().asm_formatter.add_instructions(&mut section_data);
        
            // Resets for the next file
            self.section_data = vec![];
        }

        // Generates Assembly code
        let current_source = self.code_data().current_source.clone();

        fs::create_dir_all(Path::new(&current_source).parent().unwrap())
            .unwrap();

        self.tools().asm_formatter.to_file(&Path::new(&current_source))
            .unwrap();
        self.tools().asm_formatter.reset();

        // Assembles to an object file
        platform::exec(
            linux_defaults::ASSEMBLER.to_string(),
            &[
                format!("{}/{}.asm", self.output_dir, source_path),
                "-felf64".to_string(),
                "-o".to_string(),
                format!("{}/{}.o", self.output_dir, source_path),
            ],
        );
    }

    fn link(&mut self) {
        let mut output_path = linux_defaults::OUTPUT_FILE.to_string();

        Args::when_flag('o', &self.data().options, |args_output_path: String| {
            output_path = args_output_path;
        });

        let mut args = vec!["-o".to_string(), output_path];
        
        // Maybe there are files to link within
        Args::when_flag('a', &self.data().options, |to_add: String| {
            args.push(to_add);
        });
        
        if self.data().is_library {
            args.push("-shared".to_string());
        }

        for source_path in &self.data.sources {
            args.push(format!("{}/{}.o", self.output_dir, source_path));
        }

        platform::exec(linux_defaults::LINKER.to_string(), &args);
    }

    // Data getters as it's required -------------------------------------------

    fn data(&mut self) -> &mut CompilerData {
        &mut self.data
    }

    fn tools(&mut self) -> &mut CompilerTools {
        &mut self.tools
    }

    fn code_data(&mut self) -> &mut CompilerCodeData {
        &mut self.code_data
    }
 
    fn stacks_data(&mut self) -> &mut CompilerStacksData {
        &mut self.stacks_data
    }

    // Functions for the elements ----------------------------------------------

    fn at_assembly(&mut self, code: &Token) {
        self.tools().asm_formatter.add_instruction(
            i!(Expression(code.to_string()))
        );
    }

    fn at_function(&mut self, function: &Function) {

    }

    fn at_assign(&mut self, operation: &Operation) {

    }

    fn at_plus(&mut self, operation: &Operation) {

    }
    
    fn at_minus(&mut self, operation: &Operation) {

    }
    
    fn at_multiply(&mut self, operation: &Operation) {
    
    }
    
    fn at_divide(&mut self, operation: &Operation) {

    }

    fn at_return(&mut self, value: &Token) {

    }

    fn at_variable(&mut self, variable: &Variable) {

    }

    // Other functions for Assembly code ---------------------------------------

    fn call_function(&mut self, id: &String) {
        // todo!() : Pass parameters when required.

        self.tools().asm_formatter.add_instructions(&mut vec![
            i!(Call, Op::Label(id.to_string())),
            i!(
                Mov, 
                reg!(defaults::RETURN_REGISTER), 
                reg!(defaults::FUN_RETURN_REGISTER)
            )
        ]);
    }

    fn update_return_register(&mut self, value: &Token) {
        self.tools().asm_formatter.add_instructions(&mut vec![
            i!(
                Mov, 
                reg!(defaults::RETURN_REGISTER), 
                Op::Expression(value.to_string())
            )
        ]);
    }
}

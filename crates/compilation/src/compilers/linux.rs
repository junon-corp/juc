// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::path::Path;
use x64asm::{
    formatter::Formatter, instruction as i, instruction::Instruction, label, mnemonic::Mnemonic::*,
    operand::Op, reg, register::Register::*, section, section::Section::*,
};
use jup::lang::{
    elements::{
        function::Function, 
        operation::Operation, 
        type_::Type, 
        variable::Variable
    },
    tokens::Token,
};
use args::Args;
use platform;

use crate::{
    compilers::base::Compiler, 
    data::CompilerData, 
    defaults,
    defaults::{BUILD_FOLDER, ENTRY_POINT},
    defaults::linux_defaults::*,
};

/// Compiler for 64 bits Linux platforms
pub struct LinuxCompiler {
    data: CompilerData,
    section_data: Vec<Instruction>,
}

impl LinuxCompiler {
    pub fn new(data: CompilerData) -> Self {
        Self {
            data,
            section_data: vec![],
        }
    }
}

/// SEE Functions' documentation from `Compiler` because they are not
/// written here a new time
impl Compiler for LinuxCompiler {
    fn init(&mut self) {
        if self.data().is_library {
            return;
        }

        let start_file_string = format!("{}/{}", BUILD_FOLDER, START_FILE);
        let start_file = Path::new(&start_file_string);
        std::fs::create_dir_all(start_file.parent().unwrap()).unwrap();

        let mut asm_start_file = Formatter::new(false);

        asm_start_file.add_instructions(&mut vec![
            i!(section!(Text)),
            i!(Global, Op::Label("_start".to_string())),
            i!(Extern, Op::Label(ENTRY_POINT.to_string())),
            i!(label!(START_FUNCTION.to_string())),
            i!(Call, Op::Label(ENTRY_POINT.to_string())),
            i!(Mov, reg!(Rdi), reg!(Rax)), // return of ENTRY_POINT function
            i!(Mov, reg!(Rax), Op::Literal(60)),
            i!(Syscall),
        ]);

        asm_start_file.to_file(&start_file).unwrap();

        platform::exec(
            ASSEMBLER.to_string(),
            /* arguments */
            &[
                format!("{}/{}", BUILD_FOLDER, START_FILE),
                "-f".to_string(),
                "elf64".to_string(),
                "-o".to_string(),
                format!("{}/{}.o", BUILD_FOLDER, START_FILE),
            ],
        );
    }

    fn link(&mut self) {
        let mut bin_filename: String = OUTPUT_FILE.to_string();
        Args::when_flag('o', &self.data().options, |bin_filename_: String| {
            bin_filename = bin_filename_;
        });

        let mut args = vec!["-o".to_string(), bin_filename.to_string()];
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

    fn finish_one(&mut self, source: &String) {
        // Write all static data
        self.data()
            .asm_formatter
            .add_instruction(i!(section!(Data)));

        let mut section_data = self.section_data.clone();
        self.data()
            .asm_formatter
            .add_instructions(&mut section_data);

        // Reset for the next file
        self.section_data = vec![];

        // Write assembly
        {
            let current_source = self.data().current_source.clone();
            let path = Path::new(&current_source);
            self.data().asm_formatter.to_file(&path).unwrap();
        }
        self.data().asm_formatter.reset();

        platform::exec(
            ASSEMBLER.to_string(),
            // Arguments
            &[
                format!("{}/{}.asm", BUILD_FOLDER, source),
                // Compiling to elf64 object file type
                "-f".to_string(),
                "elf64".to_string(),
                // The output is the same name than the source file but with
                // an ".o" extension
                "-o".to_string(),
                format!("{}/{}.o", BUILD_FOLDER, source),
            ],
        );
    }

    fn data(&mut self) -> &mut CompilerData {
        &mut self.data
    }

    // --- ASM code generators

    /// Define a new function in ASM code and initialize the variables' stack
    fn at_function(&mut self, function: Function) {
        // See the `Function` structure
        let id: String = function.id();
        // -- TODO --
        // Parameters and return type 

        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label(id.to_string())),
            i!(label!(id)),
            i!(Push, reg!(Rbp)),
            i!(Mov, reg!(Rbp), reg!(Rsp)),
        ]);

        self.data().i_variable_stack = 0;
    }

    /// Define a new label into `.data` section with the value
    fn at_static(&mut self, variable: Variable) {

    }

    /// Push a new variable and save its position in the variables' stack.
    /// Calls to `self.assign_variable()` 
    fn at_variable(&mut self, mut variable: Variable) {
        // See the `Variable` structure
        let id: String          = variable.id();
        let type_: Type         = variable.type_();
        let value: String       = variable.value();

        variable.set_stack_pos(self.data().i_variable_stack + type_.to_usize());
        let stack_pos: usize    = variable.stack_pos();
        self.data().i_variable_stack = stack_pos;

        self.data().variable_stack.insert(id, variable.clone());

        self.assign_variable(&variable);
    }

    /// When `operation.operator` is `Token::Assign`
    fn at_assign(&mut self, operation: &Operation) {
        let mut variable_to_assign = self.data().variable_stack
            .get_mut(&operation.arg1().to_string())
            .unwrap()
            .clone();
            
        let arg2 = operation.arg2();
        variable_to_assign.set_value(arg2.to_string());

        if arg2 == &Token::BracketOpen {
            self.execute_next_expression();
        }

        self.assign_variable(&variable_to_assign);
    }

    /// When `operation.operator` is `Token::Plus`
    fn at_plus(&mut self, operation: &Operation) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg1().to_string())),
            i!(Add, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg2().to_string())),
        ]);
    }

    fn at_return(&mut self, value: Token) {
        let instruction = if value == Token::None {
            i!(Expression("nop".to_string()))
        } else {
            if value == Token::BracketOpen {
                self.execute_next_expression();
                i!(Mov, reg!(defaults::RETURN_REGISTER), reg!(defaults::EXPRESSION_RETURN_REGISTER))
            } else {
                i!(Mov, reg!(defaults::RETURN_REGISTER), Op::Expression(value.to_string()))
            }
        };

        self.data().asm_formatter.add_instructions(&mut vec![
            instruction,
            i!(Mov, reg!(Rsp), reg!(Rbp)),
            i!(Pop, reg!(Rbp)),
            i!(Ret),
        ]);
    }

    /// Gets the variable's stack position and mov a new value at this index
    fn assign_variable(&mut self, variable: &Variable) {
        // See the `Variable` structure
        let id: String             = variable.id();
        let type_: Type            = variable.type_();
        let value: String          = variable.value();
        let stack_pos: usize       = variable.stack_pos();

        let instruction = i!(
            Mov,
            Op::Expression(format!("[rbp-{}]", stack_pos)),
            {
                if value == defaults::EXPRESSION_RETURN_REGISTER.to_string() {
                    self.execute_next_expression();
                    Op::Expression("".to_string())
                } else {
                    Op::Dword // TODO : match with variable's type
                }  
            },
            Op::Expression(value.to_string())
        )
        .with_comment(id.to_string())
        .clone();

        self.data().asm_formatter.add_instruction(instruction);
    }
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::path::Path;

use x64asm::{
    formatter::Formatter, instruction as i, instruction::Instruction, label, mnemonic::Mnemonic::*,
    operand::Op, reg, register::Register::*, section, section::Section::*,
};
use args::Args;
use objects::{function::Function, type_::Type, variable::Variable};

use platform;

use crate::{
    base::Compiler, 
    caller::Caller, 
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

impl Caller for LinuxCompiler {}

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

    /// Push a new variable and save its position in the variables' stack.
    /// Calls to `self.assign_variable()` 
    fn add_variable(&mut self, variable: Variable) {
        // See the `Variable` structure
        let id: &String             = variable.id();
        let type_: &Type            = variable.type_();
        let current_value: &String  = variable.current_value();
        let stack_pos: &usize       = variable.stack_pos();

        self.data()
            .variable_stack
            .insert(id.to_string(), variable.clone());

        self.assign_variable(&variable);
    }

    /// Define a new label into `.data` section with the value
    fn add_static_variable(&mut self, variable: Variable) {
        // See the `Variable` structure
        let id: &String             = variable.id();
        let type_: &Type            = variable.type_();
        let mut init_value: String  = variable.current_value().to_string();
        let stack_pos: &usize       = variable.stack_pos();
        
        // Auto terminate strings by NULL character
        if *variable.type_() == Type::Str && init_value != "0".to_string() {
            init_value = format!("`{}`", &init_value[1..init_value.len() - 1]);
            init_value += ", 0";
        }

        self.section_data.push(i!(
            label!(id),
            type_.to_asm_operand(),
            Op::Expression(init_value)
        ))
    }

    /// Define a new function in ASM code and initialize the variables' stack
    fn add_function(&mut self, function: Function) {
        // See the `Function` structure
        let id: &String = function.id();
        // let params
        // let return_type

        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label(id.to_string())),
            i!(label!(id)),
            i!(Push, reg!(Rbp)),
            i!(Mov, reg!(Rbp), reg!(Rsp)),
        ]);

        self.data().i_variable_stack = 0;
    }

    /// Gets the variable's stack position and mov a new value at this index
    fn assign_variable(&mut self, variable: &Variable) {
        // See the `Variable` structure
        let id: &String             = variable.id();
        let type_: &Type            = variable.type_();
        let current_value: &String  = variable.current_value();
        let stack_pos: &usize       = variable.stack_pos();

        let i_variable_stack = self.data().i_variable_stack;

        self.data().asm_formatter.add_instruction(
            i!(
                Mov,
                Op::Expression(format!("[rbp-{}]", i_variable_stack)),
                {
                    if current_value.to_string() == defaults::RETURN_REGISTER.to_string() {
                        Op::Expression("".to_string())
                    } else {
                        Op::Dword
                    }  
                },
                Op::Expression(current_value.to_string())
            )
            .with_comment(id.to_string())
            .clone()
        );
    }

    fn set_return_value(&mut self, value: String) {
        self.data().asm_formatter.add_instruction(
            i!(Mov, reg!(defaults::RETURN_REGISTER), Op::Expression(value))
        );
    }

    fn return_(&mut self, value: String) {
        self.set_return_value(value);
        
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(Rsp), reg!(Rbp)),
            i!(Pop, reg!(Rbp)),
            i!(Ret),
        ]);
    }

    fn print(&mut self, to_print: String) {
        let to_print_id = format!("_string_");

        self.add_static_variable(Variable::static_(to_print_id.clone(), Type::Str, to_print));

        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(Rdi), Op::Label(to_print_id.clone())),
            i!(Xor, reg!(Rcx), reg!(Rcx)),
            i!(Not, reg!(Rcx)),
            i!(Xor, reg!(Al), reg!(Al)),
            i!(Expression("cld".to_string())),
            i!(
                Expression("repnz".to_string()),
                Op::Expression("scasb".to_string())
            ),
            i!(Not, reg!(Rcx)),
            i!(Expression("dec".to_string()), reg!(Rcx)),
            i!(Mov, reg!(Rdx), reg!(Rcx)),
            i!(Mov, reg!(Rsi), Op::Label(to_print_id.clone())),
            i!(Mov, reg!(Rax), Op::Literal(1)),
            i!(Mov, reg!(Rdi), reg!(Rax)),
            i!(Syscall),
        ]);
    }

    fn exit(&mut self, value: String) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(Rax), Op::Literal(60)),
            i!(Mov, reg!(Rdi), Op::Expression(value)),
            i!(Syscall),
        ]);
    }
}

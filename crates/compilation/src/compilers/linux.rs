// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fs::File;
use std::io::Write;
use std::path::Path;

use x64asm::{
    instruction::Instruction,
    instruction as i, label,
    mnemonic::Mnemonic::*, 
    operand::Op, 
    reg, register::Register::*, 
    section, section::Section::*,
};

use args::Args;

use objects::{
    function::Function, 
    type_::Type, 
    variable::Variable
};

use platform;

use crate::{
    base::Compiler,
    data::CompilerData,
    defaults::linux_defaults::*,
    defaults::*,

    caller::Caller,
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

        let path: String = format!("{}/{}", BUILD_FOLDER, START_FILE);
        let path = Path::new(&path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mut file = File::create(path).unwrap();
        
        file.write_all(
            format!("section .text\n\tglobal {}\n", START_FUNCTION).as_bytes()
        ).unwrap();

        file.write_all(format!("extern {}\n", ENTRY_POINT).as_bytes()).unwrap();
        file.write_all(format!("{}:\n", START_FUNCTION).as_bytes()).unwrap();
        
        let to_write: Vec<String> = vec!(
            format!("call {}", ENTRY_POINT),
            "mov rdi, rax".to_string(), // return of ENTRY_POINT
            "mov rax, 60".to_string(),
            "syscall".to_string(),
        );
        
        file.write_all(
            to_write.iter()
                .map(| x | format!("\t{}\n", x))
                .collect::<String>()
                .as_bytes()
        ).unwrap();

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
        self.data().asm_formatter.add_instruction(i!(section!(Data)));
        
        let mut section_data = self.section_data.clone();
        self.data().asm_formatter.add_instructions(&mut section_data);

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

    fn add_variable(&mut self, variable: Variable) {
        self.data().variable_stack.insert(
            variable.id().to_string(), 
            variable.clone()
        );

        self.change_variable_value(&variable);
    }

    fn add_static_variable(&mut self, variable: Variable) {
        let mut init_value: String = variable.current_value().clone();

        // Auto terminate strings by NULL character
        if *variable.type_() == Type::Str && init_value != "0".to_string() {
            init_value = format!("`{}`", &init_value[1..init_value.len() - 1]);
            init_value += ", 0";
        }

        self.section_data.push(
            i!(
                label!(variable.id()), 
                variable.type_().to_asm_operand(), 
                Op::Expression(init_value)
            )
        )
    }

    fn add_function(&mut self, function: Function) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label(function.id().to_string())),
            i!(label!(function.id())),
            i!(Push, reg!(Rbp)),
            i!(Mov, reg!(Rbp), reg!(Rsp))
        ]);

        self.data().i_variable_stack = 0;
    }

    fn change_variable_value(&mut self, variable: &Variable) {
        let i_variable_stack = self.data().i_variable_stack;

        self.data().asm_formatter.add_instruction(
            i!(
                Mov, 
                Op::Expression(format!("[rbp-{}]", i_variable_stack)),
                Op::Dword, 
                Op::Expression(variable.current_value().to_string())
            )
            .with_comment(variable.id().to_string())
            .clone()
        );
    }

    fn return_(&mut self, value: String) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(Rax), Op::Expression(value)),
            i!(Mov, reg!(Rsp), reg!(Rbp)),
            i!(Pop, reg!(Rbp)),
            i!(Ret),
        ]);
    }

    fn print(&mut self, to_print: String) {
        let to_print_id = format!("_string_");

        self.add_static_variable(
            Variable::static_(
                to_print_id.clone(),
                Type::Str,
                to_print
            )
        );

        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(Rdi), Op::Label(to_print_id.clone())),
            i!(Xor, reg!(Rcx), reg!(Rcx)),
            i!(Not, reg!(Rcx)),
            i!(Xor, reg!(Al), reg!(Al)),
            i!(Expression("cld".to_string())),
            i!(Expression("repnz".to_string()), Op::Expression("scasb".to_string())),
            i!(Not, reg!(Rcx)),
            i!(Expression("dec".to_string()), reg!(Rcx)),
            i!(Mov, reg!(Rdx), reg!(Rcx)),
            i!(Mov, reg!(Rsi), Op::Label(to_print_id.clone())),
            i!(Mov, reg!(Rax), Op::Literal(1)),
            i!(Mov, reg!(Rdi), reg!(Rax)),
            i!(Syscall)
        ]);
    }

    fn exit(&mut self, value: String) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(Rax), Op::Literal(60)),
            i!(Mov, reg!(Rdi), Op::Expression(value)),
            i!(Syscall)
        ]);
    }
}

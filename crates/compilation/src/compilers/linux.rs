// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::path::Path;

use x64asm::{
    formatter::Formatter, 
    instruction as i, 
    instruction::Instruction, 
    label, 
    mnemonic::Mnemonic::*,
    operand::{Op, Operand},
    reg, 
    register::Register::*, 
    section, 
    section::Section::*,
};

use jup::lang::{
    elements::{
        Element,
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

/// See some Functions' documentations at `Compiler` page because they are not
/// written here again
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
        // Writes all static data
        self.data()
            .asm_formatter
            .add_instruction(i!(section!(Data)));

        let mut section_data = self.section_data.clone();
        self.data()
            .asm_formatter
            .add_instructions(&mut section_data);

        // Reset for the next file
        self.section_data = vec![];

        // Writes assembly
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

    /// Returns the expression for retrieving a value in the stack at the 
    /// variable's position
    fn give_value_of_variable(&mut self, variable: &Variable) -> String {
        format!(
            "[{}-{}]", 
            defaults::EXPRESSION_RETURN_REGISTER.to_string(),
            variable.stack_pos()
        )
    }

    fn give_operand_before_value(&mut self, value: &Token) -> Operand {
        match Self::what_kind_of_value(value) {
            "expression" | "id" => {
                Op::Expression("".to_string())
            }
            "direct" => {
                // Todo : Match the value to select the right operand : (q|d|)word
                Op::Dword
            }
            &_ => panic!(),
        }
    }

    /// Defines a new function in ASM code and initialize the variables' stack
    fn at_function(&mut self, function: Function) {
        let id: String = function.id();
        // todo!();
        // Parameters and return type 

        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label(id.to_string())),
            i!(label!(id)),
            i!(Push, reg!(Rbp)),
            i!(Mov, reg!(Rbp), reg!(Rsp)),
        ]);

        self.data().i_variable_stack = 0;
    }

    /// Defines a new label into `.data` section with the value
    fn at_static(&mut self, variable: Variable) {

    }

    /// Push a new variable and save its position into the variables' stack.
    ///
    /// Calls to `self.assign_variable()` 
    fn at_variable(&mut self, mut variable: Variable) {
        // See the `Variable` structure
        let id: String          = variable.id();
        let type_: &Type        = variable.type_();
        let value: &Token       = variable.value();

        variable.set_stack_pos(self.data().i_variable_stack + type_.to_usize());
        let stack_pos: usize    = variable.stack_pos();
        self.data().i_variable_stack = stack_pos;

        self.data().variable_stack.insert(id, variable.clone());

        self.assign_variable(&variable);
    }

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

    fn at_plus(&mut self, operation: &Operation) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg1().to_string())),
            i!(Add, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg2().to_string())),
        ]);
    }
    
    fn at_minus(&mut self, operation: &Operation) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg1().to_string())),
            i!(Sub, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg2().to_string())),
        ]);
    }

    fn at_multiply(&mut self, operation: &Operation) {
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Mov, reg!(defaults::EXPRESSION_RETURN_REGISTER), Op::Expression(operation.arg1().to_string())),
            i!(Mov, reg!(Rdx), Op::Expression(operation.arg2().to_string())),
            i!(Mul, reg!(Rdx)),
        ]);
    }

    fn at_divide(&mut self, operation: &Operation) {
        // todo!();
    }

    fn at_return(&mut self, value: Token) {
        let instruction = if value == Token::None {
            i!(Expression("nop".to_string()))
        } else {
            i!(
                Mov, 
                reg!(defaults::RETURN_REGISTER), 
                Op::Expression(self.give_value(&value))
            )
        };

        self.data().asm_formatter.add_instructions(&mut vec![
            instruction,
            i!(Pop, reg!(Rbp)),
            i!(Ret),
        ]);
    }

    /// Gets the variable's stack position and mov a new value at this index
    ///
    /// Calls to `assign_array_variable` when the value is an array
    fn assign_variable(&mut self, variable: &Variable) {
        let value: &Token       = variable.value();

        match value {
            Token::SquareBracketOpen => {
                self.assign_array_variable(variable);
                return;
            }
            Token::None => return,
            _ => {}
        }

        let id: String          = variable.id();

        let instruction = i!(
            Mov,
            Op::Expression(self.give_value_of_variable(variable)),
            self.give_operand_before_value(value),
            Op::Expression({
                // Here, we don't use completely `give_value()` because it also
                // executes the expression and it's not needed. 
                if value == &Token::BracketOpen {
                    defaults::EXPRESSION_RETURN_REGISTER.to_string()
                } else {
                    // `value` cannot be `Token::BracketOpen`
                    self.give_value(value)
                }
            })
        )
        .with_comment(id.to_string())
        .clone();

        self.data().asm_formatter.add_instruction(instruction);
    }

    fn assign_array_variable(&mut self, array: &Variable) {
        let array_values: Vec<Token> = match self.data().next_element.clone() {
            Element::Array(values) => values,
            _ => panic!() // never happens
        };
        
        // Array's type is the type for each value
        let id: String              = array.id();
        let stack_pos: usize    = array.stack_pos();
        
        let (element_type, length) = match array.type_() {
            Type::Array(type_, length) => (*type_.clone(), length),
            Type::StaticArray(type_) => (*type_.clone(), &0),
            _ => panic!() // never happens
        };   

        for (i, value) in array_values.iter().enumerate() {
            let mut value_as_variable = Variable::new(
                Token::Other(format!("{}[{}]", id, i)),
                element_type.clone(),
                value.clone()
            );

            value_as_variable.set_stack_pos(
                stack_pos - element_type.to_usize() * i
            );
            
            self.assign_variable(&value_as_variable);
        }
    }

    fn at_other(&mut self, other: Token) {
        if other == Token::NewLine {
            return;
        }

        self.data().asm_formatter.add_instruction(
            i!(Mov, reg!(Rbx), Op::Expression(other.to_string()))
        );
    }
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::path::Path;

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

        let mut to_add = String::new();
        Args::when_flag('a', &self.data().options, |to_add_: String| {
            to_add = to_add_;
        });

        let mut args = vec!["-o".to_string(), bin_filename.to_string()];
        if !to_add.is_empty() {
            args.push(to_add);
        }

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
            println!("{:?}", path);
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
            Register::Rbp.to_string(),
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

    fn give_operand_for_value(&mut self, value: &Token) -> (Operand, bool) {
        if Self::what_kind_of_value(value) == "id" {
            let value_as_variable = self.data().variable_stack
                .get(&value.to_string())
                .unwrap()
                .clone();
            (
                Op::Expression(self.give_value_of_variable(&value_as_variable)), 
                true
            )
        } else {
            (Op::Expression(self.give_value(value)), false)
        }
    }

    /// According to https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-170
    fn give_register_for_param(&mut self, _param: Element, param_i: usize) -> Operand {
        match param_i {
            0 => reg!(Rcx),
            1 => reg!(Rdx),
            2 => reg!(R8),
            3 => reg!(R9),
            _ => {
                self.data().i_parameter_stack += 4; // TODO : decltype
                Op::Expression(format!("[rsp-{}]", self.data().i_parameter_stack))
            }
        }
    }

    /// When the value is in fact a variable's id, moves the value to the 
    /// expression return register, and assign the expression default register 
    /// to retrieve the value of the variable
    fn before_getting_value_when_id(&mut self, value: &Token, to_register: Register) {
        if Self::what_kind_of_value(value) != "id" {
            return;
        }

        let instruction = i!(
            Mov,
            reg!(to_register),
            {
                let value_as_variable = self.data().variable_stack
                    .get(&value.to_string())
                    .unwrap()
                    .clone();
                
                Op::Expression(self.give_value_of_variable(
                    &value_as_variable
                ))
            }
        );

        self.data().asm_formatter.add_instruction(instruction);
    }

    fn at_assembly(&mut self, code: Token) {
        self.data().asm_formatter.add_instruction(i!(
            Expression(code.to_string())
        ));
    }

    fn at_call(&mut self, fun_to_call: &String) {
        // Pass parameters when required
        match self.data().next_element.clone() {
            Element::Parameters(params) => {
                let mut stack_parameters_counter: usize = 0;
                let mut param_i: usize = 0;

                for element in params {
                    // Todo : #[derive(PartialEq, Eq)] in jup
                    //
                    // if element == Element::Other(Token::Comma) {
                    //     continue;
                    // }
                    match element {
                        Element::Other(Token::Comma) => continue,
                        _ => {}
                    }

                    let instruction = i!(
                        Mov,
                        self.give_register_for_param(element.clone(), param_i),
                        match element {
                            Element::Expression(_) => reg!(defaults::EXPRESSION_RETURN_REGISTER),
                            Element::Other(value) => {
                                self.give_operand_for_value(&value).0
                            }
                            _ => panic!()
                        }
                    );

                    self.data().asm_formatter.add_instruction(instruction);
                    param_i += 1;
                }
            }
            _ => {}
        }
        
        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Call, Op::Label(fun_to_call.to_string())),
            i!(Mov, reg!(defaults::EXPRESSION_RETURN_REGISTER), reg!(defaults::RETURN_REGISTER))
        ]);
    }

    /// Defines a new function in ASM code and initialize the variables' stack
    fn at_function(&mut self, function: Function) {
        let id: String = function.id();

        self.data().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label(id.to_string())),
            i!(label!(id)),
            i!(Push, reg!(Rbp)),
            i!(Mov, reg!(Rbp), reg!(Rsp)),
        ]);

        let params = match function.params() {
            Element::Parameters(elements) => elements,
            _ => panic!()
        };

        // Retrieves given parameters when needed
        if !params.is_empty() {
            let mut current_param = Variable::new(
                Token::Other("".to_string()),
                Type::None, 
                Token::None
            );

            let mut stack_parameters_counter: usize = 0;
            let mut param_i: usize = 0;

            for element in params {                
                match element {
                    Element::Other(ref token) => match token.clone() {
                        Token::TypeDef | Token::Comma => continue,
                        Token::Other(ref value) => {
                            if current_param.id().is_empty() {
                                current_param.set_id(token.clone());
                                param_i += 1;
                            } else {
                                current_param.set_type(Type::from_string(value.clone()));
                                
                                self.data().variable_stack.insert(
                                    current_param.id(), 
                                    current_param.clone()
                                );

                                current_param.set_stack_pos(self.data().i_variable_stack + current_param.type_().to_usize());
                                let stack_pos: usize = current_param.stack_pos();
                                self.data().i_variable_stack = stack_pos;

                                let instruction = i!(
                                    Mov,
                                    Op::Expression(self.give_value_of_variable(&current_param)),
                                    self.give_register_for_param(element, param_i)
                                );
                                self.data().asm_formatter.add_instruction(instruction);

                                current_param = Variable::new(
                                    Token::Other("".to_string()),
                                    Type::None, 
                                    Token::None
                                );
                            }
                        },
                        _ => panic!()
                    }
                    _ => panic!()
                }
            }
        }

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

    fn arithmetic_operation(&mut self, operation: &Operation, operation_mnemonic: Mnemonic) {
        let (arg1_value, arg1_is_id) 
            = self.give_operand_for_value(operation.arg1());
        
        let (arg2_value, arg2_is_id) 
            = self.give_operand_for_value(operation.arg2());

        let mut instructions = if arg1_is_id && arg2_is_id {
            vec![
                i!(
                    Mov, 
                    reg!(defaults::EXPRESSION_RETURN_REGISTER_2), 
                    arg1_value
                ),
                i!(
                    Mov, 
                    reg!(defaults::EXPRESSION_RETURN_REGISTER), 
                    arg2_value
                ),
                i!(
                    operation_mnemonic,
                    reg!(defaults::EXPRESSION_RETURN_REGISTER), 
                    reg!(defaults::EXPRESSION_RETURN_REGISTER_2)
                )
            ]
        } else {
            vec![
                i!(
                    Mov, 
                    reg!(defaults::EXPRESSION_RETURN_REGISTER), 
                    arg1_value
                ),
                i!(
                    operation_mnemonic, 
                    reg!(defaults::EXPRESSION_RETURN_REGISTER), 
                    arg2_value
                ),
            ]
        };

        self.data().asm_formatter.add_instructions(&mut instructions);
    }

    fn at_plus(&mut self, operation: &Operation) {
        self.arithmetic_operation(operation, Mnemonic::Add);
    }
    
    fn at_minus(&mut self, operation: &Operation) {
        self.arithmetic_operation(operation, Mnemonic::Sub);
    }

    fn at_multiply(&mut self, operation: &Operation) {
        // todo!();
    }

    fn at_divide(&mut self, operation: &Operation) {
        // todo!();
    }

    fn at_return(&mut self, value: Token) {
        let instruction = if value == Token::None {
            i!(Expression("nop".to_string()))
        } else {
            self.before_getting_value_when_id(
                &value, 
                defaults::EXPRESSION_RETURN_REGISTER
            );
            
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

        self.data().i_variable_stack = 0;
        self.data().i_parameter_stack = 0;
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

        self.before_getting_value_when_id(
            value, 
            defaults::EXPRESSION_RETURN_REGISTER
        );

        let instruction = i!(
            Mov,
            Op::Expression(self.give_value_of_variable(variable)),
            self.give_operand_before_value(value),
            Op::Expression({
                // Here, we don't use completely `give_value()` because it also
                // executes the expression and it's not needed. 
                if value == &Token::BracketOpen {
                    self.execute_next_expression();
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
        self.data().asm_formatter.add_instruction(
            i!(Mov, reg!(Rbx), Op::Expression(other.to_string()))
        );
    }
}

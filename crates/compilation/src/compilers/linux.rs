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
    instruction as i, 
    instruction::Instruction, 
    label, 
    mnemonic::{
        Mnemonic,
        Mnemonic::*,
    },
    operand::{Op, Operand},
    reg, 
    register::{
        Register,
        Register::*,
    },
    section, 
    section::Section::*,
};

use args::Args;
use platform;

use crate::{
    compilers::base::{ Compiler, KindToken },
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

    pub fn give_value(&mut self, id_or_value_or_expression: &Token) -> Operand {
        match KindToken::from_token(id_or_value_or_expression) {
            KindToken::Expression => {
                self.execute_next_expression();
                reg!(defaults::RETURN_REGISTER)
            },
            KindToken::Identifier => {
                let variable_from_id = self.stacks_data().variable_stack
                    .get(&id_or_value_or_expression.to_string())
                    .unwrap()
                    .clone();

                self.give_expression_for_variable(&variable_from_id)
            }
            KindToken::Value => Op::Expression(id_or_value_or_expression.to_string()),
        }
    }

    /// Gives the expression for variable in stack to gets its value
    pub fn give_expression_for_variable(&mut self, variable: &Variable) -> Operand {
        Op::Expression(format!(
            "[{}-{}]", 
            Register::Rbp.to_string(), 
            variable.stack_pos()
        ))
    }

    /// Gives the right register for the current parameter from its index
    pub fn give_register_for_parameter(&mut self, i_parameter: usize) -> Operand {
        match i_parameter {
            0 => reg!(Rcx),
            1 => reg!(Rdx),
            2 => reg!(R8),
            3 => reg!(R9),
            _ => todo!("only 4 arguments can be given"),
        }
    }

    pub fn give_type_operand_before_value(&mut self, variable: &Variable, id_or_value_or_expression: &Token) -> Operand {
        match KindToken::from_token(id_or_value_or_expression) {
            KindToken::Expression | KindToken::Identifier => {
                // Nothing because we move a register 
                Op::None
            },
            KindToken::Value => {
                match variable.type_() {
                    Type::Byte => Op::Byte,
                    Type::Integer => Op::Word,
                    Type::BigInteger => Op::Dword,
                    type_ => panic!("no possible operand for type : {:?}", type_),
                }
            }
        }
    }

    /// Moves the value, when `id_or_value` is the variable's id, to the 
    /// expression return register, and assign the expression default register 
    /// to retrieve the value of the variable
    ///
    /// So, "id_or_value" should be named "id" here
    ///
    /// Returns if it was an identifier or not
    fn before_getting_value_when_id(&mut self, id_or_value: &Token, to_register: Register) -> bool {        
        // Not an identifier, nothing to do
        if KindToken::from_token(id_or_value) != KindToken::Identifier {
            return false;
        }

        let instruction = i!(
            Mov,
            reg!(to_register),
            {
                let value_as_variable = self.stacks_data().variable_stack
                    .get(&id_or_value.to_string())
                    .unwrap()
                    .clone();
                
                self.give_expression_for_variable(&value_as_variable)
            }
        );

        self.tools().asm_formatter.add_instruction(instruction);
        return true;
    }

    fn do_arithmetic_operation(&mut self, operation: &Operation, operation_mnemonic: Mnemonic) {
        let arg1_value = self.give_value(operation.arg1());
        let arg2_value = self.give_value(operation.arg2());

        let is_arg1_identifier = KindToken::from_token(operation.arg1()) == KindToken::Identifier;
        let is_arg2_identifier = KindToken::from_token(operation.arg2()) == KindToken::Identifier;

        let mut instructions = if is_arg1_identifier && is_arg2_identifier {
            vec![
                i!(
                    Mov, 
                    reg!(defaults::RETURN_REGISTER_2), 
                    arg1_value
                ),
                i!(
                    Mov, 
                    reg!(defaults::RETURN_REGISTER), 
                    arg2_value
                ),
                i!(
                    operation_mnemonic,
                    reg!(defaults::RETURN_REGISTER), 
                    reg!(defaults::RETURN_REGISTER_2)
                )
            ]
        } else {
            vec![
                i!(
                    Mov, 
                    reg!(defaults::RETURN_REGISTER), 
                    arg1_value
                ),
                i!(
                    operation_mnemonic, 
                    reg!(defaults::RETURN_REGISTER), 
                    arg2_value
                ),
            ]
        };

        self.tools().asm_formatter.add_instructions(&mut instructions);
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
    
        self.tools().asm_formatter.add_instruction(i!(section!(Text)));
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

    /// If `condition` token == `Token::None`, the condition is terminated even 
    /// without "else" closure.
    fn at_condition(&mut self, condition: &Token) {        
        match condition {
            Token::ConditionIf => {
                self.code_data().is_condition = true;
                self.code_data().n_condition += 1;

                self.execute_next_expression();
                
                let instruction = i!(
                    self.give_mnemonic_for_previous_operator(),
                    Op::Label(format!(
                        ".cond_{}", self.code_data().n_condition
                    ))
                );
                
                self.tools().asm_formatter.add_instruction(instruction);
            }
            Token::ConditionElse => {
                let mut instructions = vec![ 
                    i!(
                        Jmp,
                        Op::Label(format!(
                            ".cond_{}", self.code_data().n_condition + 1
                        ))
                    ),
                    i!(
                        label!(format!(
                            ".cond_{}", self.code_data().n_condition
                        ).as_str())
                    )
                ];

                self.code_data().n_condition += 1;

                self.tools().asm_formatter.add_instructions(&mut instructions);
                self.execute_next_expression();
                self.code_data().is_condition = false;
            }
            Token::None => {
                self.code_data().is_condition = false;
            }
            _ => panic!("not a condition token : {:?}", condition)
        }

        if !self.code_data().is_condition {
            let instruction = i!(label!(
                format!(".cond_{}", self.code_data().n_condition)
            ));
            self.tools().asm_formatter.add_instruction(instruction);
        }
    }

    fn at_function(&mut self, function: &Function) {
        if function.id().to_string() == defaults::ENTRY_POINT.to_string() {
            self.create_start_function();
        }
        
        // Creates a label for the function.
        //
        // Initializes the stacks.
        self.tools().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label(function.id().to_string())),
            i!(label!(function.id())),
            i!(Push, reg!(Rbp)),
            i!(Mov, reg!(Rbp), reg!(Rsp)),
        ]); 

        // Retrieves the function's parameters because it's an `Element`.
        // 
        // The panic! will never happen.
        let parameters = match function.params() {
            Element::Parameters(elements) => elements,
            _ => panic!("parameters are not a parameters element"),
        };

        // Prepares the variable stack iterator for the function 
        self.stacks_data().i_variable_stack = 0;
        
        // There is no parameters to retrieve, returns
        if parameters.is_empty() {
            return;
        }

        // Retrieves passed parameters to variables
        let mut i_parameter: usize = 0;
        // Creates an empty variable that will be filled step by step in the
        // code below
        let mut current_parameter = Variable::new(Token::None, Type::None, Token::None);

        for element in parameters {
            let token = match element {
                Element::Other(token) => token,
                _ => panic!("passed parameter is not valid : {:?}", element)
            };

            let id_or_type = match token {
                Token::TypeDef | Token::Comma => continue,
                Token::Other(ref id_or_type) => id_or_type,
                _ => panic!("invalid token found in parameter list : {:?}", token)
            };
            
            // Sets parameter's id
            if current_parameter.id().is_empty() {
                current_parameter.set_id(token.clone());
                i_parameter += 1;
                continue;
            }

            current_parameter.set_type(Type::from_string(id_or_type.clone()));
            
            current_parameter.set_stack_pos(
                self.stacks_data().i_variable_stack + 
                    current_parameter.type_().to_usize()
            );
            // Inserts the finished variable into the stack
            // The value is set as `Token::None` because it will be changed in
            // Assembly code, not here
            self.stacks_data().variable_stack.insert(
                current_parameter.id(),
                current_parameter.clone()
            );

            let stack_position: usize = current_parameter.stack_pos();
            self.stacks_data().i_variable_stack = stack_position;

            // Creates the variable associated to the parameter, in Assembly 
            // with the passed value
            let instruction = i!(
                Mov,
                self.give_expression_for_variable(&current_parameter),
                self.give_register_for_parameter(i_parameter - 1)
            );

            self.tools().asm_formatter.add_instruction(instruction);

            current_parameter = Variable::new(Token::None, Type::None, Token::None);
        }
    }

    fn at_loop(&mut self, token: &Token) {
        println!("{:?}", token);

        match token {
            Token::Loop => {
                self.code_data().is_loop = true;
                self.code_data().n_loop += 1;

                let instruction = i!(
                    label!(format!(
                        ".loop_test_{}",
                        self.code_data().n_loop
                    ).as_str())
                );
                self.tools().asm_formatter.add_instruction(instruction);

                self.execute_next_expression();
                                
                let mut instructions = vec![
                    i!(
                        self.give_mnemonic_for_previous_operator(),
                        Op::Label(format!(
                            ".loop_core_{}",
                            self.code_data().n_loop
                        ))
                    ),
                    i!(
                        Jmp,
                        Op::Label(format!(
                            ".loop_end_{}",
                            self.code_data().n_loop
                        ))
                    ),
                    i!(
                        label!(format!(
                            ".loop_core_{}",
                            self.code_data().n_loop
                        ).as_str())
                    )
                ];

                self.tools().asm_formatter.add_instructions(&mut instructions);   
            }
            Token::LoopBreak => {
                let instruction = i!(
                    Jmp,
                    Op::Label(format!(
                        ".loop_end_{}",
                        self.code_data().n_loop
                    ))
                );
                self.tools().asm_formatter.add_instruction(instruction);
                self.code_data().is_loop = false;
            }
            Token::LoopContinue => {
                let instruction = i!(
                    Jmp,
                    Op::Label(format!(
                        ".loop_test_{}",
                        self.code_data().n_loop
                    ))
                );

                self.tools().asm_formatter.add_instruction(instruction);
            }
            Token::None => {
                self.code_data().is_loop = false;
            }
            _ => panic!("not a loop token : {:?}", token),
        }

        if !self.code_data().is_loop {
            let mut instructions = vec![
                i!(
                    Jmp,
                    Op::Label(format!(
                        ".loop_test_{}",
                        self.code_data().n_loop
                    ))
                ),
                i!(
                    label!(format!(
                        ".loop_end_{}",
                        self.code_data().n_loop
                    ).as_str())
                ),
            ];

            self.tools().asm_formatter.add_instructions(&mut instructions);
        }
    }

    /// Retrieves the variable to assign and calls `self.assign_variable()` with
    /// the value next to the operator
    /// ```
    /// <arg1> <operator> <arg2>
    /// variable_to_assign = value
    /// ```
    fn at_assign(&mut self, operation: &Operation) {
        let mut variable_to_assign = self.stacks_data().variable_stack
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

    fn at_addition(&mut self, operation: &Operation) {
        self.do_arithmetic_operation(operation, Mnemonic::Add);
    }
    
    fn at_subtraction(&mut self, operation: &Operation) {
        self.do_arithmetic_operation(operation, Mnemonic::Sub);
    }
    
    fn at_multiply(&mut self, operation: &Operation) {
        let mut instructions = vec![
            i!(Mov, reg!(Rax), self.give_value(operation.arg1())),
            i!(Imul, reg!(Rax), self.give_value(operation.arg2())),
            i!(Mov, reg!(Rbx), reg!(Rax))
        ];
        self.tools().asm_formatter.add_instructions(&mut instructions);
    }
    
    fn at_divide(&mut self, operation: &Operation) {
        let mut instructions = vec![
            i!(Mov, reg!(Rax), self.give_value(operation.arg1())),
            i!(Mov, reg!(Rdx), self.give_value(operation.arg2())),
            i!(Idiv, reg!(Rax)),
            i!(Mov, reg!(Rbx), reg!(Rax))
        ];
        self.tools().asm_formatter.add_instructions(&mut instructions);
    }

    fn at_comparison(&mut self, operation: &Operation) {
        let mut instructions = vec![
            i!(Mov, reg!(Rbx), self.give_value(operation.arg1())),
            i!(Cmp, reg!(Rbx), self.give_value(operation.arg2())),
        ];
        
        let mnemonic = match operation.operator() {
            Token::Equal => Sete,
            Token::LessThan => Setl,
            Token::MoreThan => Setg,
            Token::LessThanOrEqual => Setle,
            Token::MoreThanOrEqual => Setge, 
            token => panic!("not a comparison operator : {:?}", token)
        };

        // Actual code to retrieves the comparison value
        if !self.code_data().is_condition && !self.code_data().is_loop {
            instructions.push(i!(mnemonic, reg!(Al)));
            instructions.push(i!(Movzx, reg!(defaults::RETURN_REGISTER), reg!(Al)));
        }

        self.tools().asm_formatter.add_instructions(&mut instructions);
        self.code_data().previous_operator = operation.operator().clone();
    }

    /// Moves the value to return into the default function return register
    ///
    /// Terminates the stacks and returns the Assembly function
    fn at_return(&mut self, id_or_value: &Token) {
        let instruction = if id_or_value == &Token::None {
            i!(
                Xor, 
                reg!(defaults::FUN_RETURN_REGISTER), 
                reg!(defaults::FUN_RETURN_REGISTER)
            )
        } else {
            let instruction = i!(
                Mov, 
                reg!(defaults::FUN_RETURN_REGISTER), 
                self.give_value(id_or_value)
            );

            instruction
        };

        self.tools().asm_formatter.add_instructions(&mut vec![
            instruction,
            i!(Pop, reg!(Rbp)),
            i!(Ret),
        ]);

        self.stacks_data().i_variable_stack = 0;
        self.stacks_data().i_parameter_stack = 0;
    }

    /// Sets the stack position for the variable and assign it with the value.
    ///
    /// Pushes the variable object into the variable stack for the compiler
    fn at_variable(&mut self, variable: &Variable) {
        let mut variable = variable.clone();

        variable.set_stack_pos(
            self.stacks_data().i_variable_stack + variable.type_().to_usize()
        );

        self.stacks_data().i_variable_stack = variable.stack_pos();
        self.stacks_data().variable_stack.insert(variable.id(), variable.clone());

        self.assign_variable(&variable);
    }

    // Other functions for Assembly code ---------------------------------------

    fn create_start_function(&mut self) {
        self.tools().asm_formatter.add_instructions(&mut vec![
            i!(Global, Op::Label("_start".to_string())),
            i!(label!("_start".to_string())),
            i!(Call, Op::Label(defaults::ENTRY_POINT.to_string())),
            i!(Mov, reg!(Rdi), reg!(defaults::FUN_RETURN_REGISTER)),
            i!(Mov, reg!(Rax), Op::Literal(60)),
            i!(Syscall),
        ]);
    }

    fn call_function(&mut self, id: &String) {
        let parameters = match self.code_data().next_element.clone() {
            Element::Parameters(parameters) => parameters,
            _ => panic!("passed parameters should be a parameters element"),
        };

        let mut i_parameter: usize = 0;

        for element in parameters {
            let operand = match element {
                Element::Expression(_) => reg!(defaults::RETURN_REGISTER),
                Element::Other(Token::Comma) => continue,
                Element::Other(id_or_value_or_expression) => {
                    self.give_value(&id_or_value_or_expression)
                },
                _ => panic!("passed parameter is not valid : {:?}", element),
            };

            let instruction = i!(
                Mov,
                self.give_register_for_parameter(i_parameter),
                operand
            );

            self.tools().asm_formatter.add_instruction(instruction);
            i_parameter += 1;
        }

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
        let instruction = i!(
            Mov, 
            reg!(defaults::RETURN_REGISTER), 
            self.give_value(&value)
        );

        self.tools().asm_formatter.add_instruction(instruction);
    }

    /// The value to assign is the value stored in the variable object.
    ///
    /// Detects when the value to assign is an array and calls 
    /// `self.assign_array_variable()` instead.
    ///
    /// When nothing to assign, just ignore.
    fn assign_variable(&mut self, variable: &Variable) {
        // Detection for arrays and no value
        match variable.value() {
            Token::SquareBracketOpen => {
                self.assign_array_variable(variable);
                return;
            },
            Token::None => return,
            _ => {}
        }

        let is_identifier: bool = self.before_getting_value_when_id(
            variable.value(), 
            defaults::RETURN_REGISTER
        );

        let mut instruction = i!(
            Mov,
            self.give_expression_for_variable(variable),
            self.give_type_operand_before_value(&variable, variable.value()),
            {
                if variable.value() == &Token::BracketOpen {
                    // Here, we don't use `self.give_value()` because it also 
                    // executes the expression and it's not required.
                    self.execute_next_expression();
                    Op::Expression(defaults::RETURN_REGISTER.to_string())
                } else if is_identifier {
                    // When the value to assign is a variable's identifier.
                    // 
                    // The variable's value was moved into the register, now
                    // we move that register into the variable to assign
                    reg!(defaults::RETURN_REGISTER)
                } else {
                    // `value` cannot be `Token::BracketOpen` so it's not an 
                    // expression
                    self.give_value(variable.value())
                }
            }
        );

        self.tools().asm_formatter.add_instruction(
            instruction.with_comment(variable.id().to_string()).clone()
        );
    }

    fn assign_array_variable(&mut self, array_variable: &Variable) {
        let values: Vec<Token> = match self.code_data().next_element.clone() {
            Element::Array(values) => values,
            _ => panic!("try to assign a non-array element to an array variable"),
        };

        let (element_type, length) = match array_variable.type_() {
            Type::Array(type_, length) => (type_.clone(), length),
            Type::StaticArray(type_) => (type_.clone(), &0),
            _ => panic!("assign a non-array variable"),
        };   

        for (i, value) in values.iter().enumerate() {
            let mut value_as_variable = Variable::new(
                Token::Other(format!("{}[{}]", array_variable.id(), i)),
                *element_type.clone(),
                value.clone()
            );

            value_as_variable.set_stack_pos(
                array_variable.stack_pos() - element_type.to_usize() * i
            );
            
            self.assign_variable(&value_as_variable);
        }
    }

    fn give_mnemonic_for_previous_operator(&mut self) -> Mnemonic {
        match &self.code_data().previous_operator {
            Token::Equal => Jne,
            Token::LessThan => Jge,
            Token::MoreThan => Jle,
            Token::LessThanOrEqual => Jg,
            Token::MoreThanOrEqual => Jl, 
            token => panic!("not a comparison operator : {:?}", token)
        }
    }
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use jup::lang::tokens::Token;

use x64asm::{
    ddirective, ddirective::DefineDirective::*, 
    formatter::Formatter, 
    instruction as i, label,
    mnemonic::Mnemonic::*, 
    operand::Op, 
    reg, register::Register::*, 
    section, section::Section::*,
};

use objects::{
    function::Function,
    type_, 
    type_::Type,
    variable::Variable,
};

use crate::base;



/// All implementations for tokens \
/// SEE `parsing::token`
pub trait Caller {
    fn when_assembly_code(&mut self) -> usize
    where Self: base::Compiler
    {
        // Assembly code is contained in only one token since this commit :
        // https://github.com/junon-corp/jup/commit/75c51f43e7a1bc5633b0e514cb733a4bb39c5a2b
        let assembly_code: &Token 
            = &self.data().current_parsed.clone()[self.data().i_current_token];

        self.data().asm_formatter.add_instruction(
            i!(Expression(assembly_code.to_string()))
        );

        return 1;
    }

    /// If the previous token is not an known identifier, it raises an error \
    /// Need the `line` and not "next_tokens" because it uses a token placed
    /// before the assign token  
    fn when_assign(&mut self) -> usize
    where Self: base::Compiler
    {
        // let mut reversed_line_iter = line.into_iter().rev();
        
        // let value: String = match reversed_line_iter.next().unwrap() {
        //     Token::Other(value) => value,
        //     _ => panic!(), // never happens
        // };
    
        // reversed_line_iter.next(); // Token::Assign

        // let to_assign = match reversed_line_iter.next().unwrap() {
        //     Token::Other(identifier) => identifier,
        //     _ => todo!(), // not an identifier
        // };

        // let mut variable = match self.data().variable_stack.get(&to_assign) {
        //     Some(v) => v.clone(),
        //     None => panic!(), // invalid identifier
        // };
        // variable.set_current_value(value);
        // self.change_variable_value(&variable);
        return 0;
    }
    
    fn when_function(&mut self) -> usize
    where Self: base::Compiler 
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        let id: String = next_tokens_iter.next().unwrap().to_string();
        to_skip += 1;
 
        self.data().current_scope.push(id.to_string());
        let current_scope_copy = self.data().current_scope.clone();
        
        if id == "main" {
            self.data().current_scope.reset();
            self.data().current_scope.push("main".to_string());
        }

        let function = Function::new(
            self.data().current_scope.to_string(),
            // TODO :
            vec![], // params
            String::new(), // return type
        );

        self.add_function(function);
        self.data().current_scope = current_scope_copy;

        to_skip
    }

    fn when_return(&mut self) -> usize
    where Self: base::Compiler
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        // Only implemented with "ret <value>" and not for an expression or
        // multiple values
        self.return_(
            match next_tokens_iter.next().unwrap() {
                // It could be a number, a `Other` does not mean that it's a 
                // string object
                Token::Other(return_value) => {
                    to_skip += 1;
                    return_value.to_string()
                }
                Token::NewLine => String::from("0"), // "null" value
                _ => panic!()
            }
        );

        self.data().current_scope.pop();
        to_skip
    }

    fn when_static(&mut self) -> usize 
    where Self: base::Compiler 
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        let (type_, current_value) = self.retrieve_variable_info(&mut to_skip);

        let static_variable = Variable::static_(
            next_tokens_iter.next().unwrap().to_string(),
            type_.unwrap(),
            current_value,
        );
        
        self.add_static_variable(static_variable);
        to_skip
    }

    fn when_variable(&mut self) -> usize 
    where Self: base::Compiler 
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        let (type_, current_value) = self.retrieve_variable_info(&mut to_skip);
        self.data().i_variable_stack += type_.as_ref().unwrap().to_usize();

        let variable = Variable::new(
            next_tokens_iter.next().unwrap().to_string(),
            type_.unwrap(),
            current_value,
            self.data().i_variable_stack.clone()
        );
        to_skip += 1;
        self.add_variable(variable);
        
        to_skip
    }

    fn retrieve_variable_info(&mut self, to_skip: &mut usize) -> (Option<Type>, String) 
    where Self: base::Compiler
    {
        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        let mut type_: Option<Type> = None;
        let mut current_value = "0".to_string();

        let mut previous_token = Token::None;
        for token in next_tokens_iter {
            match &previous_token {
                Token::Assign => {
                    current_value = match token {
                        Token::Other(value_as_string) => value_as_string,
                        _ => panic!(), // never happens
                    }.to_string();
                }
                Token::TypeDef => {
                    type_ = Some(Type::from_string(match token {
                        Token::Other(type_as_string) => type_as_string,
                        _ => panic!(), // never happens
                    }.to_string()));
                }
                Token::None => {} // first token
                Token::Other(_variable_id) => {} 
                Token::NewLine => break,
                _ => panic!() // never happens
            }
            if previous_token != Token::None {
                *to_skip += 1;
            }

            previous_token = token.clone();
        }

        (type_, current_value)
    }

    fn when_print(&mut self) -> usize 
    where Self: base::Compiler
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        if next_tokens.len() == 0 {
            self.print(String::new());
        }

        let mut to_print: Vec<String> = vec![];
        for token in next_tokens_iter.next() {
            match token {
                // It could be a number, a `Other` does not mean that it's a 
                // string object
                Token::Other(x) => to_print.push(x.to_string()),
                _ => panic!(), // never happens
            }
        }

        for x in to_print {
            self.print(x);
        }
        
        to_skip
    }

    fn when_exit(&mut self) -> usize
    where Self: base::Compiler
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[
            self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        // Only implemented with "exit <value>" and not for an expression or
        // multiple values
        self.exit(
            match next_tokens_iter.next().unwrap() {
                // It could be a number, a `Other` does not mean that it's a 
                // string object
                Token::Other(exit_value) => exit_value.to_string(),
                Token::NewLine => String::from("0"), // "null" value
                _ => panic!(),
            }
        );
        return 0;
    }
}
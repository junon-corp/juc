// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::fmt;

use crate::junon::compilation::{
    objects::{
        function::Function,
        type_, 
        type_::Type,
        variable::Variable,
    },
    parsing::{
        tokens,
        tokens::Token,
    },
    base,
};

/// All implementations for tokens \
/// SEE `parsing::token`
pub trait Caller {
    fn when_assembly_code(&mut self, next_tokens: Vec<Token>) 
    where Self: base::Compiler
    {    
        let mut next_tokens_as_asm = String::from("\t");

        for token in next_tokens {
            next_tokens_as_asm += &match token {
                Token::Comma => ", ".to_string(),
                Token::RawString(string) => format!("{} ", string),
                _ => panic!() // never happens
            };
        }

        self.write_asm(next_tokens_as_asm);
    }

    fn when_assign(&mut self, _next_tokens: Vec<Token>) {

    }
    
    fn when_function(&mut self, _next_tokens: Vec<Token>) 
    where Self: base::Compiler 
    {
        let function = Function::new(
            tokens::token_to_string(&self.data().current_token),
            // TODO :
            vec![], // params
            String::new(), // return type
        );

        self.data().current_scope += format!("::{}", function.id()).as_str();
        self.add_function(function);
    }

    fn when_return(&mut self, next_tokens: Vec<Token>)
    where Self: base::Compiler 
    {
        // Only implemented with "ret <value>" and not for an expression or
        // multiple values
        self.return_(match next_tokens.iter().next().unwrap() {
            // It could be a number, a `RawString` does not mean that it's a 
            // string object
            Token::RawString(return_value) => return_value.to_string(),
            _ => panic!()
        });
    }

    fn when_static(&mut self, next_tokens: Vec<Token>) 
    where Self: base::Compiler 
    {
        let mut type_: Option<Type> = None;
        let mut current_value = "0".to_string();

        let mut previous_token = Token::None;
        for token in next_tokens.iter() {
            match previous_token {
                Token::Assign => {
                    current_value = match token {
                        Token::RawString(value_as_string) => value_as_string,
                        _ => panic!(), // never happens
                    }.to_string();
                }
                Token::TypeDef => {
                    type_ = Some(type_::string_to_type(match token {
                        Token::RawString(type_as_string) => type_as_string,
                        _ => panic!(), // never happens
                    }.to_string()));
                }
                Token::None => {} // first token
                _ => panic!() // never happens
            }
            previous_token = token.clone();
        }

        let static_variable = Variable::static_(
            tokens::token_to_string(&self.data().current_token),
            type_.unwrap(),
            current_value,
        );
    }

    fn when_variable(&mut self, _next_tokens: Vec<Token>) {

    }

    fn when_other(&mut self) {
        // panic!() // never happens
    }
}
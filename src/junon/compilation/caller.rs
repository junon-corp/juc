// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::fmt;

use crate::junon::compilation::{
    objects::function::Function,
    parsing::tokens::Token,
    base,
};

/// All implementations for tokens \
/// SEE `parsing::token`
pub trait Caller {
    fn when_assembly_code(&mut self, next_tokens: Vec<Token>) 
    where Self: base::Compiler
    {    
        let mut next_tokens_as_asm = String::from("\t");
        for token in next_tokens.iter() {
            next_tokens_as_asm += match token {
                Token::RawString(string) => format!("{} ", string),
                _ => panic!() // never happens
            }.as_str();
        }
        self.write_asm(next_tokens_as_asm);
    }

    fn when_assign(&mut self, _next_tokens: Vec<Token>) {

    }

    fn when_comment(&mut self, _next_tokens: Vec<Token>) {

    }
    
    fn when_function(&mut self, _next_tokens: Vec<Token>) 
    where Self: base::Compiler {
        let function = Function::new(
            match &self.data().current_token {
                Token::RawString(func_name) => func_name.to_string(),
                _ => panic!() // never happens, the function's identifier cannot
                // be a reserved word by Junon
            },

            // TODO :
            vec![], // params
            String::new(), // return type
        );

        self.add_function(function);
    }

    fn when_return(&mut self, _next_tokens: Vec<Token>) {

    }

    fn when_static(&mut self, _next_tokens: Vec<Token>) {

    }

    fn when_variable(&mut self, _next_tokens: Vec<Token>) {

    }

    fn when_other(&mut self) {
        // panic!() // never happens
    }
}
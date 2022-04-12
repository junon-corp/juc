// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

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

        let first = next_tokens.first().unwrap().clone();

        for token in next_tokens {
            next_tokens_as_asm += &tokens::token_to_string(&token);
            
            // Do not put space when it's a function label
            match first {
                Token::RawString(ref string) => {
                    match string.as_str() {
                        "call" | "jmp" => if first != token { continue; },
                        _ => {}
                    }
                },
                _ => {}
            }

            next_tokens_as_asm += &" ";
        }

        self.write_asm(next_tokens_as_asm);
    }

    /// If the previous token is not an known identifier, it raises an error \
    /// Need the `line` and not "next_tokens" because it uses a token placed
    /// before the assign token  
    fn when_assign(&mut self, line: Vec<Token>) 
    where Self: base::Compiler 
    {
        let mut reversed_line_iter = line.into_iter().rev();
        
        let value: String = match reversed_line_iter.next().unwrap() {
            Token::RawString(value) => value,
            _ => panic!(), // never happens
        };
    
        reversed_line_iter.next(); // Token::Assign

        let to_assign = match reversed_line_iter.next().unwrap() {
            Token::RawString(identifier) => identifier,
            _ => todo!(), // not an identifier
        };

        let mut variable = match self.data().variable_stack.get(&to_assign) {
            Some(v) => v.clone(),
            None => panic!(), // invalid identifier
        };
        variable.set_current_value(value);
        self.change_variable_value(&variable);
    }
    
    fn when_function(&mut self, _next_tokens: Vec<Token>) 
    where Self: base::Compiler 
    {
        let id: String = tokens::token_to_string(&self.data().current_token);
 
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
    }

    fn when_return(&mut self, next_tokens: Vec<Token>)
    where Self: base::Compiler 
    {
        // Only implemented with "ret <value>" and not for an expression or
        // multiple values
        self.return_(match next_tokens.iter().next() {
            Some(token) => match token {
                // It could be a number, a `RawString` does not mean that it's a 
                // string object
                Token::RawString(return_value) => return_value.to_string(),
                _ => panic!(), // never happens
            }
            None => String::from("0"), // "null" value
        });

        self.data().current_scope.pop();
    }

    fn when_static(&mut self, next_tokens: Vec<Token>) 
    where Self: base::Compiler 
    {
        let (type_, current_value) 
            = self.retrieve_variable_info(next_tokens);

        let static_variable = Variable::static_(
            tokens::token_to_string(&self.data().current_token),
            type_.unwrap(),
            current_value,
        );
        self.add_static_variable(static_variable);
    }

    fn when_variable(&mut self, next_tokens: Vec<Token>) 
    where Self: base::Compiler 
    {
        let (type_, current_value) = 
            self.retrieve_variable_info(next_tokens);
        
        self.data().i_variable_stack 
            += type_::type_size_to_usize(&type_.as_ref().unwrap());

        let variable = Variable::new(
            tokens::token_to_string(&self.data().current_token),
            type_.unwrap(),
            current_value,
            self.data().i_variable_stack.clone()
        );
        self.add_variable(variable);
    }

    fn retrieve_variable_info(
        &mut self, 
        next_tokens: Vec<Token>
    ) -> (Option<Type>, String) 
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
                Token::RawString(_variable_id) => {} 
                _ => panic!() // never happens
            }
            previous_token = token.clone();
        }

        (type_, current_value)
    }

    fn when_other(&mut self) {
        // panic!() // never happens
    }
}
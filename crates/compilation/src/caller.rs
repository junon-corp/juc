// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use jup::lang::tokens::Token;
use x64asm::{instruction as i, label, mnemonic::Mnemonic::*};
use objects::{function::Function, type_::Type, variable::Variable};
use crate::{base, defaults};

/// All implementations for tokens \
/// SEE `parsing::token`
pub trait Caller {
    fn when_assembly_code(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        // Assembly code is contained in only one token since this commit :
        // https://github.com/junon-corp/jup/commit/75c51f43e7a1bc5633b0e514cb733a4bb39c5a2b
        let assembly_code: &Token =
            &self.data().current_parsed.clone()[self.data().i_current_token];

        self.data()
            .asm_formatter
            .add_instruction(i!(Expression(assembly_code.to_string())));

        return 1;
    }

    /// If the previous token is not an known identifier, it raises an error \
    /// Need the `line` and not "next_tokens" because it uses a token placed
    /// before the assign token  
    fn when_assign(&mut self) -> usize
    where
        Self: base::Compiler,
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
        // self.assign_variable(&variable);
        return 0;
    }

    fn when_expression(&mut self) -> usize 
    where
        Self: base::Compiler,
    {
        self.data().i_current_expr = self.data().i_current_token;

        let mut expr_tokens = &self.data()
            .current_parsed
            .clone()[self.data().i_current_token..];

        let mut i = 0;
        let mut sub_expression = false;
        for token in expr_tokens.iter() {
            if *token == Token::BracketOpen {
                sub_expression = true;
            }
            if !sub_expression && *token == Token::BracketClose {
                sub_expression = false;
                break;
            }
            i += 1;
        }

        expr_tokens = &expr_tokens[..i];

        println!("{} | new expr -> {:?}", self.data().i_current_token, expr_tokens);

        self.call(&expr_tokens.to_vec());

        let mut return_value = Token::None;

        for token in expr_tokens.iter().rev() {
            if *token != Token::NewLine {
                return_value = token.clone();
                break;
            }
        }

        println!("\x1b[36m{:?}\x1b[0m", return_value);
        match return_value {
            Token::Other(x) => self.set_return_value(x),
            _ => {} // no return value given
        }
        self.data().current_scope.pop();

        expr_tokens.len()
    }

    fn when_function(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data()
            .current_parsed
            .clone()[self.data().i_current_token..];
            
        let mut next_tokens_iter = next_tokens.iter();
        // ---
        // println!("FUNCTION with {:?}", next_tokens_iter);

        let id: String = next_tokens_iter.next().unwrap().to_string();
        to_skip += 1;

        self.data().current_scope.push(id.to_string());
        let current_scope_save = self.data().current_scope.clone();

        if id == "main" {
            self.data().current_scope.reset();
            self.data().current_scope.push("main".to_string());
        }

        let function = Function::new(
            self.data().current_scope.to_string(),
            // TODO :
            vec![],        // params
            String::new(), // return type
        );

        self.add_function(function);
        self.data().current_scope = current_scope_save;

        to_skip
    }

    fn when_return_(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data().current_parsed.clone()[self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        let mut return_value = "0".to_string();

        let next = next_tokens_iter.clone().next();
        if next != Some(&Token::NewLine) && next != Some(&Token::BracketClose) {
            return_value = self.direct_value_or_expression(
                next_tokens_iter.clone(), 
                &mut to_skip
            );
        }

        self.return_(return_value);

        to_skip
    }

    fn when_static(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        let mut to_skip: usize = 0;

        // let next_tokens = &self.data().current_parsed.clone()[self.data().i_current_token..];
        // let mut next_tokens_iter = next_tokens.iter();
        // // ---

        // let type_ = Type::from_string(next_tokens_iter.next().unwrap().to_string());

        // let static_variable = Variable::static_(
        //     next_tokens_iter.next().unwrap().to_string(),
        //     type_,
        //     "0".to_string(),
        // );

        // self.add_static_variable(static_variable);
        to_skip
    }

    fn when_variable(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        let mut to_skip: usize = 0;

        let next_tokens = &self.data()
            .current_parsed
            .clone()[self.data().i_current_token..];
        let mut next_tokens_iter = next_tokens.iter();
        // ---

        let id: String = next_tokens_iter.next().unwrap().to_string();
        next_tokens_iter.next(); // Token::TypeDef
        let type_ = Type::from_string(next_tokens_iter.next().unwrap().to_string());
        let mut init_value = "0".to_string();
        to_skip += 2;

        println!("'{}' : '{:?}'", id, type_);

        // Cloned because don't want to modify the iterator if it's a variable
        // without assigned value
        let mut next = next_tokens_iter.clone().next();

        if next == Some(&Token::Assign) {
            // `Token::Assign` is here, we have to skip it
            next_tokens_iter.next();
            to_skip += 1;

            init_value = self.direct_value_or_expression(
                next_tokens_iter, 
                &mut to_skip
            );
        }

        println!("init_value : {}", init_value);

        self.data().i_variable_stack += type_.to_usize();

        let variable = Variable::new(
            id,
            type_,
            init_value,
            self.data().i_variable_stack.clone(),
        );

        self.add_variable(variable);

        to_skip
    }

    fn when_print(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        let mut to_skip: usize = 0;

        // let next_tokens = &self.data().current_parsed.clone()[self.data().i_current_token..];
        // let mut next_tokens_iter = next_tokens.iter();
        // // ---

        // if next_tokens.len() == 0 {
        //     self.print(String::new());
        // }

        // let mut to_print: Vec<String> = vec![];
        // for token in next_tokens_iter.next() {
        //     match token {
        //         // It could be a number, a `Other` does not mean that it's a
        //         // string object
        //         Token::Other(x) => to_print.push(x.to_string()),
        //         _ => panic!(), // never happens
        //     }
        // }

        // for x in to_print {
        //     self.print(x);
        // }

        to_skip
    }

    fn when_exit(&mut self) -> usize
    where
        Self: base::Compiler,
    {
        let mut to_skip: usize = 0;

        // let next_tokens = &self.data().current_parsed.clone()[self.data().i_current_token..];
        // let mut next_tokens_iter = next_tokens.iter();
        // // ---

        // // Only implemented with "exit <value>" and not for an expression or
        // // multiple values
        // self.exit(match next_tokens_iter.next().unwrap() {
        //     // It could be a number, a `Other` does not mean that it's a
        //     // string object
        //     Token::Other(exit_value) => exit_value.to_string(),
        //     Token::NewLine => String::from("0"), // "null" value
        //     _ => panic!(),
        // });

        to_skip
    }

    /// Gives the value or the return register according to what's next
    ///
    /// When there is no value or expression next, "0" is returned (it happens
    /// in situations like `let a: int` without `= ?`) 
    /// 
    /// If an expression has been found,`self.when_expression()` is called
    fn direct_value_or_expression<'a, T>(
        &mut self, 
        mut next_tokens_iter: T, 
        to_skip: &mut usize
    ) -> String
    where
        Self: base::Compiler,
        T: Iterator<Item = &'a Token>,
    {
        // Default value when nothing set
        let mut ret = "0".to_string();

        for next in next_tokens_iter {
            // Skip all new lines to find a value or an expression
            if *next == Token::NewLine {
                *to_skip += 1;
                continue;
            }

            // Check for value or expression
            ret = match next {
                Token::BracketOpen => {
                    self.data().i_current_token += *to_skip + 2;
                    *to_skip += self.when_expression() + 2;
                    self.data().i_current_token -= *to_skip;

                    defaults::RETURN_REGISTER.to_string()
                }
                Token::Other(value) => {
                    *to_skip += 2;
                    value.to_string()
                },
                _ => panic!() // never happens
            };
            break;
        }

        ret
    }
}

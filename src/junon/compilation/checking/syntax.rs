// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::{
    compilation::{
        checking::{
            data::CheckerData,
            base::Checker,
        },
        parsing::{
            tokens,
            tokens::Token,
        },
        data::CompilerData,
    },
    logger::*,
};

/// Check syntax of a given source file content
pub struct SyntaxChecker {
    data: CheckerData, 
}

impl SyntaxChecker {
    pub fn new(data: CheckerData) -> Self {
        Self {
            data,
        }
    }
}

/// SEE Functions' documentation from `Checker` because they are not
/// written here a new time
impl Checker for SyntaxChecker {
    fn init(&mut self) {

    }

    fn check(&mut self) {
        let parsed = self.data().parsed.clone();

        for line in parsed.iter() {
            let mut previous_token = Token::None;
            let mut break_line = false; // to break the loop from the closure

            for token in line.iter() {
                if break_line {
                    break;
                }

                self.check_for_instruction(
                    line, 
                    &mut break_line, 
                    &token,
                    &mut previous_token
                );
                previous_token = token.clone();
            }
        }
    }

    fn check_for_instruction(
        &mut self, 
        line: &Vec<Token>, 
        break_line: &mut bool,
        token: &Token,
        previous_token: &mut Token
    ) {
        let mut line_iter_for_next_tokens = line.iter();
        line_iter_for_next_tokens.next();

        match previous_token {
            Token::Variable => {
                println!("var: {:?}", token);
                // self.data().logger.add_log(Log::new(
                //     LogLevel::Error,
                //     "Expected token".to_string(),
                //     format!(
                //         "No token was found next token to {:?} but expected",
                //         token   
                //     )
                // ));
                // return;
                
                let mut error = false;
                match token {
                    Token::RawString(variable_id) => {
                        match variable_id.parse::<i64>() {
                            Ok(_) => error = true,
                            Err(_) => {},
                        }
                    }
                    _ => error = true,
                }
                
                if error {
                    self.data().logger.add_log(Log::new(
                        LogLevel::Error,
                        "Invalid identifier for variable".to_string(),
                        format!(
                            "{}Found '{}' but it cannot be used as a variable identifier",
                            line_to_string(line),
                            tokens::token_to_string(token)
                        )
                    ));
                }

                *break_line = true;
            }
            // Because there is no token before the line's first token
            Token::None => {},
            _ => {},
        }
    }

    fn finish(&mut self) {
        self.data().logger.interpret();
    }

    fn data(&mut self) -> &mut CheckerData {
        &mut self.data
    }
}
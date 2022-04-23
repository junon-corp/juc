// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use jup::tokens::Token;

use crate::junon::{
    compilation::{
        checking::{
            data::CheckerData,
            base::Checker,
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

            self.data().token_i = 0;
            for token in line.iter() {
                if break_line {
                    break;
                }

                self.check_for_instruction(
                    line, 
                    &mut break_line, 
                    &token,
                    &mut previous_token,
                );
                previous_token = token.clone();
                self.data().token_i += 1;
            }
            self.data().line_i += 1;
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

        let cause: String = source_to_string(
            self.data().source.clone(), 
            self.data().line_i.clone(), 
            self.data().token_i.clone(),
        );

        let token_i: usize = self.data().token_i.clone();

        match previous_token {
            Token::Assembly => {
                *break_line = true;
            },
            Token::Assign => {},
            Token::Function => {},
            Token::Return => {},
            Token::Variable | Token::Static => {
                // Check for an identifier         
                let mut error = false;
                match token {
                    Token::Other(variable_id) => {
                        match variable_id.parse::<i64>() {
                            Ok(_) => error = true,
                            Err(_) => {},
                        }
                    }
                    _ => error = true,
                }

                if error {
                    self.data().logger.add_log(
                        Log::new(
                            LogLevel::Error,
                            "Invalid identifier for variable".to_string(),
                            format!(
                                "{}Found '{}' but it cannot be used as a variable identifier",
                                line_to_string(line, token_i + 1),
                                token.to_string()
                            )
                        )
                        .add_cause(cause.clone())
                        .finish()
                    );
                    *break_line = true;
                    return;
                }

                // Check for type definition
                line_iter_for_next_tokens.next();
                let next = line_iter_for_next_tokens.next();
                match next {
                    Some(token) => match token {
                        Token::TypeDef => {}
                        _ => {
                            self.data().logger.add_log(
                                Log::new(
                                    LogLevel::Error,
                                    "Invalid token".to_string(),
                                    format!(
                                        "{}Found '{}' but it's not a type definition token",
                                        line_to_string(line, token_i + 2),
                                        token.to_string()
                                    )
                                )
                                .add_cause(cause)
                                .finish()
                            )
                        }
                    }
                    None => {
                        self.data().logger.add_log(
                            Log::new(
                                LogLevel::Error,
                                "Expected token".to_string(), 
                                format!(
                                    "{}No token was found next to '{}' but expected",
                                    line_to_string(line, token_i + 1),
                                    token.to_string()
                                )
                            )
                            .add_cause(cause)
                            .add_hint(format!(
                                "Specify the variable's type with '{}' + <type>",
                                &Token::TypeDef.to_string()
                            ))
                        )
                    }
                }

                *break_line = true;
            }

            Token::Print => *break_line = true,
            Token::Exit => *break_line = true,

            // First token of the line
            Token::None => {
                // Lonely token
                if line.len() == 1 {
                    match token {
                        Token::Variable | Token::Static => {},
                        _ => return,
                    }

                    self.data().logger.add_log(
                        Log::new(
                            LogLevel::Error,
                            "Expected token".to_string(),
                            format!(
                                "{}No token was found next to '{}' but expected",
                                line_to_string(line, token_i + 1),
                                token.to_string()
                            )
                        )
                        .add_cause(cause)
                        .finish()
                    );
                }
            },
            _ => {
                self.data().logger.add_log(
                    Log::new(
                        LogLevel::Error,
                        "Invalid token instruction".to_string(),
                        format!(
                            "{}No valid instruction found for token '{}'",
                            line_to_string(line, token_i),
                            previous_token.to_string()
                        )
                    )
                    .add_cause(cause)
                    .finish()
                );
            },
        }
    }

    fn finish(&mut self) -> Result<(), Logger> {
        self.data().logger.get_result()
    }

    fn data(&mut self) -> &mut CheckerData {
        &mut self.data
    }
}

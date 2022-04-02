// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::fmt;
use std::fs::File;
use std::io::Read;

use crate::junon::{
    compilation::parsing::tokens::*, 
    logger::*
};

pub struct Parser {
    parsed: Vec<Vec<Token>>,
    source: Option<String>,
    content: String,
}

impl fmt::Debug for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.parsed {
            if line.len() < 1 {
                write!(f, "\n")?;
            } else {
                write!(f, "{:?}\n", line)?;
            }
        }
        Ok(())
    }
}

impl Parser {
    pub fn new(source: &String) -> Self {
        Self {
            parsed: vec![],
            source: Some(source.to_string()),
            content: String::new(),
        }
    }

    pub fn from(content: String) -> Self {
        Self {
            parsed: vec![],
            source: None,
            content,
        }
    }

    pub fn run(&mut self) {
        match self.source {
            None => {} // content is not from a source file
            _ => self.read_file_content(),
        }

        let mut token = String::new();
        let mut line: Vec<Token> = vec![];

        let mut was_double_char = false;
        let mut is_string = false;
        let mut string_content = String::new();

        let mut is_asm_code = false;
        let mut is_comment = false;

        for (i, c) in self.content.chars().enumerate() {
            if c != '\n' && is_comment {
                continue;
            }

            // String creation

            // Get the first character because it's a String of one character
            if c == token_to_string(&Token::StringDot).chars().nth(0).unwrap() {
                if is_string {
                    // ending of string
                    is_string = false;
                    line.push(string_to_token(
                        &format!("\"{}\"", string_content)
                    ));
                    string_content = String::new(); // reset
                } else {
                    // beginning of string
                    is_string = true;
                }
                continue;
            }
            if is_string {
                string_content.push(c);
                continue; // don't care of others possibilities, we want raw
                          // characters in the String
            }

            // The user directly wrote ASM code
            if c == '@' {
                token = "@".to_string();
                Self::push_token(&mut token, &mut line, is_asm_code);
                is_asm_code = true;
                continue;
            }

            // New line detected
            if c == '\n' {
                // Push the last token of the line
                Self::push_token(&mut token, &mut line, is_asm_code);

                // Push the line into the parsed 2d list
                if line != vec![] { // can be if it was a comment
                    self.parsed.push(line.clone());
                    line = vec![]; // reset line
                }

                is_asm_code = false;
                is_comment = false;

                continue; // and then '\n' will be not pushed
            }

            // When it's a special character (not letter or number, not simple
            // point).
            // SEE `tokens::should_be_cut()`
            if should_be_cut(&c) {
                Self::push_token(&mut token, &mut line, is_asm_code);

                // And push the special character detected as a new token
                if c != ' ' && !was_double_char {
                    if i != self.content.len() - 1 
                        && c == self.content.chars().nth(i + 1).unwrap()
                    {
                        let double_char_as_token = string_to_token(
                            &format!("{}{}", c, c)
                        );
                        if double_char_as_token == Token::Comment {
                            is_comment = true;
                            continue;
                        }
                        line.push(double_char_as_token);
                        
                        was_double_char = true;
                        continue;
                    }

                    if is_asm_code {
                        let token_string = format!("{}", c);
                        if string_to_token(&token_string) == Token::Comma {
                            line.push(Token::Comma);
                        } else {
                            line.push(Token::RawString(format!("{}", c)));
                        }
                    } else {
                        line.push(string_to_token(&format!("{}", c)));
                    }
                }
                was_double_char = false;
                continue;
            }

            token.push(c); // it's still the same "word"/"token"
        }

        if token != "" {
            line.push(string_to_token(&token));
        }
        if line != vec![] {
            self.parsed.push(line.clone());
        }
    }

    /// Push the token into the line whenever it's not a null token.
    /// The given `token` name parameter is a String, and it is converted into
    /// a `Token` before pushing.
    fn push_token(token: &mut String, line: &mut Vec<Token>, is_asm_code: bool) {
        if *token != String::new() {
            if is_asm_code {
                line.push(Token::RawString(token.clone()));
            } else {
                line.push(string_to_token(&token));
            }
            *token = String::new(); // reset
        }
    }

    /// Update the `content` attribute to the file's content by opening a new
    /// file stream (readable) on it.
    fn read_file_content(&mut self) {
        let mut stream = File::open(self.source.as_ref().unwrap())
            .unwrap(); // already checked before

        match stream.read_to_string(&mut self.content) {
            Err(_) => {
                let mut logger = Logger::new();

                logger.add_log(
                    Log::new(
                        LogLevel::Error,
                        "Unreadable file".to_string(),
                        "The given source file cannot be read".to_string(),
                    )
                    .add_hint("It's probably corrupted or it's not a text file".to_string()),
                );

                logger.interpret();
            }
            Ok(_) => {}
        }
    }

    pub fn parsed(&self) -> &Vec<Vec<Token>> {
        &self.parsed
    }
}

/// NOTE you should run the test with parameters: "-- --nocapture" to see the
/// outputs of the logs
#[test]
fn test1() {
    let mut parser = Parser::from("a b c func".to_string());
    parser.run();
    println!("output:\n{:?}", parser);

    let mut parser = Parser::from("a // f\n//g".to_string());
    parser.run();
    println!("output:\n{:?}", parser);

    let mut parser = Parser::from("\"abc\" func \"def\"".to_string());
    parser.run();
    println!("output:\n{:?}", parser);

    let mut parser = Parser::from("\"abc\"".to_string());
    parser.run();
    println!("output:\n{:?}", parser);
}

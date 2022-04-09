// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    AssemblyCode,
    Assign,
    Comma,
    Comment,
    Function,
    ParenOpen,
    ParenClose,
    Return,
    Static,
    StringDot,
    TypeDef,
    Variable,

    // Example : "foo" or something that is not a real token
    RawString(String),

    None, // to avoid the usage of an `Option`
}

/// Get a `Token` enum object from the name as String
/// SEE `token_to_string()` (reversed function)
pub fn string_to_token(token_name: &String) -> Token {
    match token_name.as_str() {
        "@" => Token::AssemblyCode,
        "=" => Token::Assign,
        "," => Token::Comma,
        "//" => Token::Comment,
        "func" => Token::Function,
        "(" => Token::ParenOpen,
        ")" => Token::ParenClose,
        "ret" => Token::Return,
        "static" => Token::Static,
        "\"" => Token::StringDot,
        ":" => Token::TypeDef,
        "let" => Token::Variable,
        
        _ => Token::RawString(token_name.clone())
    }
}

/// Get the name as String of a `Token` enum object
/// SEE `string_to_token()` (reversed function)
pub fn token_to_string(token: &Token) -> String {
    match *token {
        Token::AssemblyCode => "@",
        Token::Assign => "=",
        Token::Comma => ",",
        Token::Comment => "//",
        Token::Function => "func",
        Token::ParenOpen => "(",
        Token::ParenClose => ")",
        Token::Return => "ret",
        Token::Static => "static",
        Token::StringDot => "\"",
        Token::Variable => "let",
        Token::TypeDef => ":",
        Token::RawString(ref val) => &*val,
        
        Token::None => panic!(), // never happens
    }
    .to_string()
}

/// If the character is special (it means that it's not a letter from the Latin
/// alphabet or if it's not a number), it return "true": the character should be
/// cut by the parser in a new case (should be not placed with the previous
/// character/word)
pub fn should_be_cut(c: &char) -> bool {
    !c.is_alphanumeric()
}

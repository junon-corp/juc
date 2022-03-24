// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Token {
    // Programming words
    Copy,
    Closure,
    Exit,
    Expression,
    ForLoop,
    Function,
    Get,
    In,
    Loop,
    Match,
    Pass,
    Return,
    Type,
    Use,
    Variable,
    AssemblyCode,

    True,
    False,
    Infinite,

    // Programming operators
    Assign1,
    Assign2,
    If,
    Elif,
    Else,
    IsType,
    CanRaiseError,
    Into,
    OutOf,
    Comma,

    // Position for `in ... get ...<pos>`
    Position,       // th
    PositionFirst,  // st
    PositionSecond, // nd
    PositionThird,  // rd

    // Math operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo, // SEE https://en.wikipedia.org/wiki/Modulo_operation

    // Comparison operators
    And,
    Or,
    Equal,
    Less,
    More,
    LessOrEq,
    MoreOrEq,

    // Pointer and references
    DeRef,
    Reference,
    CPointer,

    // Characters
    OpenParen,      // (
    CloseParen,     // )
    OpenBrace,      // {
    CloseBrace,     // }
    OpenBracket,    // [
    CloseBracket,   // ]

    MemberOf,
    FromPackage,

    StringDot,
    CharacterDot,

    // Comments
    Comment,
    DocComment,
    
    CommentNote,
    CommentWarn,
    CommentTodo,
    CommentSee,

    // Something that is not a token
    RawString(Box<str>),
}

/// Get a `Token` enum object from the name as String
/// SEE `get_string_token()` (reversed function)
pub fn get_token(token_name: &String) -> Token {
    use Token::*;

    match token_name.as_str() {
        // Programming words
        "cpy" => Copy,
        "end" => Closure,
        "exit" => Exit,
        "expr" => Expression,
        "for" => ForLoop,
        "func" => Function,
        "get" => Get,
        "in" => In,
        "loop" => Loop,
        "match" => Match,
        "..." => Pass,
        "ret" => Return,
        "type" => Type,
        "use" => Use,
        "let" => Variable,
        "@" => AssemblyCode,

        "true" => True,
        "false" => False,
        "infinite" => Infinite,

        // Programming operators
        "=" => Assign1,
        "eq" => Assign2,
        "if" => If,
        "elif" => Elif,
        "else" => Else,
        ":" => IsType,
        "?" => CanRaiseError,
        "<<" => Into,
        ">>" => OutOf,
        "," => Comma,

        // Position for `in ... get ...<pos>`
        "th" => Position,
        "st" => PositionFirst,
        "nd" => PositionSecond,
        "rd" => PositionThird,

        // Math operators
        "+" => Plus,
        "-" => Minus,
        "*" => Multiply,
        "/" => Divide,
        "%" => Modulo,  // SEE https://en.wikipedia.org/wiki/Modulo_operation

        // Comparison operators
        "and" => And,
        "or" => Or,
        "==" => Equal,
        "<" => Less,
        ">" => More,
        "<=" => LessOrEq,
        ">=" => MoreOrEq,

        // Pointer and references
        "$" => DeRef,
        "&" => Reference,
        "c_ptr" => CPointer,

        // Characters
        "(" => OpenParen,
        ")" => CloseParen,
        "{" => OpenBrace,
        "}" => CloseBrace,
        "[" => OpenBracket,
        "]" => CloseBracket,

        "." => MemberOf,
        "::" => FromPackage,

        "\"" => StringDot,
        "'" => CharacterDot,

        // Comments
        "//" => Comment,
        "///" => DocComment,
        
        "NOTE" => CommentNote,
        "WARN" => CommentWarn,
        "TODO" => CommentTodo,
        "SEE" => CommentSee,

        _ => RawString(token_name.clone().into_boxed_str()),
    }
}

/// Get the name as String of a `Token` enum object
/// SEE `get_token()` (reversed function)
pub fn get_string_token(token: Token) -> String {
    use Token::*;

    match token {
        // Programming words
        Copy => "cpy",
        Closure => "end",
        Exit => "exit",
        Expression => "expr",
        ForLoop => "for",
        Function => "func",
        Get => "get",
        In => "in",
        Loop => "loop",
        Match => "match",
        Pass => "...",
        Return => "ret",
        Type => "type",
        Use => "use",
        Variable => "let",
        AssemblyCode => "@",

        True => "true",
        False => "false",
        Infinite => "infinite",

        // Programming operators
        Assign1 => "=",
        Assign2 => "eq",
        If => "if",
        Elif => "elif",
        Else => "else",
        IsType => ":",
        CanRaiseError => "?",
        Into => "<<",
        OutOf => ">>",
        Comma => ",",

        // Position for `in ... get ...<pos>`
        Position => "th",    
        PositionFirst => "st",  
        PositionSecond => "nd", 
        PositionThird => "rd",  

        // Math operators
        Plus => "+",
        Minus => "-",
        Multiply => "*",
        Divide => "/",
        Modulo => "%", // SEE https://en.wikipedia.org/wiki/Modulo_operation

        // Comparison operators
        And => "and",
        Or => "or", 
        Equal => "==",
        Less => "<",
        More => ">",
        LessOrEq => "<=",
        MoreOrEq => ">=",

        // Pointer and references
        DeRef => "*",
        Reference => "&",
        CPointer => "c_ptr",

        // Characters
        OpenParen => "(",
        CloseParen => ")",
        OpenBrace => "{",
        CloseBrace => "}",
        OpenBracket => "[",
        CloseBracket => "]",

        MemberOf => ".",
        FromPackage => "::",

        StringDot => "\"",
        CharacterDot => "'",

        // Comments
        Comment => "//",
        DocComment => "///",
        
        CommentNote => "NOTE",
        CommentWarn => "WARN",
        CommentTodo => "TODO",
        CommentSee => "SEE",

        RawString(ref val) => &*val,
    }.to_string()
}

/// If the character is special (it means that it's not a letter from the Latin 
/// alphabet or if it's not a number), it return "true": the character should be
/// cut by the parser in a new case (should be not placed with the previous 
/// character/word)
pub fn should_be_cut(c: &char) -> bool {
    if (*c >= 'A' && *c <= 'Z') || (*c >= 'a' && *c <= 'z') {
        false
    } else if (*c >= '0' && *c <= '9') || (*c == '.') {
        false
    } else {
        true
    }
}

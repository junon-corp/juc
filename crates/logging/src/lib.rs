// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use jup::tokens::Token;

pub mod level;
pub mod log;
pub mod logger;

/// Transform a line of tokens to a printable string for a log
pub fn line_to_string(line: &Vec<Token>, token_i: usize) -> String {
    let mut result = String::from("\t");
    
    let mut i = 0;
    for token in line {
        if token_i > 0 && i == token_i - 1 {
            result += &"\x1b[31m";
        }
        result += &format!("{}\x1b[0m ", token.to_string());
        i += 1;
    }
    result += "\n\n";
    result
}

pub fn source_to_string(source: String, line_i: usize, token_i: usize) -> String {
    format!("in '{}' at ({}, {})", source, line_i + 1, token_i + 1)
}

/// NOTE you should run the test with parameters: "-- --nocapture" to see the
/// outputs of the logs
#[test]
fn test() {
    let mut logger = Logger::new();

    let logs: Vec<Log> = vec![
        Log::info("This is an info log".to_string()),
        Log::new(
            LogLevel::Error,
            "This is an error log".to_string(),
            "This message is for this error log".to_string(),
        ),
        Log::new(
            LogLevel::Warning,
            "This is a warning log".to_string(),
            "This message is for this warning log".to_string(),
        ),
        Log::new(
            LogLevel::Error,
            "This is an error log with an hint".to_string(),
            "This message is for this error log".to_string(),
        )
        .add_hint("This hint is for this error log".to_string()),
        Log::new(
            LogLevel::Error,
            "This is an error log with a cause and an hint".to_string(),
            "This message is for this error log".to_string(),
        )
        .add_cause("This cause is for this error log".to_string())
        .add_hint("This hint is for this error log".to_string()),
        Log::info("This is another info log".to_string()),
    ];

    for log in logs {
        logger.add_log(log);
    }

    logger.interpret();
}

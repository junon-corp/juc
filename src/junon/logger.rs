// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fmt;
use std::process;

use crate::junon::compilation::parsing::{
    tokens,
    tokens::Token,
};

#[derive(Clone, PartialEq)]
pub enum LogLevel {
    Error,
    Warning,
    Info, // only printed in Debug mode
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                LogLevel::Error => "\x1b[1;31mError",
                LogLevel::Warning => "\x1b[1;33mWarning",
                LogLevel::Info => "\x1b[1;34mInfo",
            }
        )
    }
}

#[derive(Clone)]
pub struct Log {
    level: LogLevel,

    title: String,
    cause: String,
    message: String,
    hint: String,
}

impl Log {
    pub fn new(level: LogLevel, title: String, message: String) -> Self {
        Log {
            level,

            title,
            cause: String::new(),
            message,
            hint: String::new(),
        }
    }

    /// An debug information does not only need a title because it's a simple
    /// message (without `cause` or `hint` for example) \
    /// This constructor permits to create easily an info log
    pub fn info(title: String) -> Self {
        Log {
            level: LogLevel::Info,
            title,

            cause: String::new(),
            message: String::new(),
            hint: String::new(),
        }
    }

    /// Add a cause that raise the problem, in the log content (the point of
    /// this log)
    ///
    /// NOTE The cause will be not printed if this function is not called
    pub fn add_cause(&mut self, cause: String) -> &mut Self {
        if self.cause != String::new() {
            self.cause += ", ";
        }
        self.cause += cause.as_str();
        self
    }

    /// Add something that could help the user, in the log content
    ///
    /// NOTE The hint will be not printed if this function is not called
    pub fn add_hint(&mut self, hint: String) -> Self {
        self.hint += hint.as_str();
        self.clone()
    }

    pub fn finish(&mut self) -> Self {
        self.clone()
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.level == LogLevel::Info && !cfg!(debug_assertions) {
            return Ok(());
        }

        let mut to_write = format!("{}: {}\x1b[0m", self.level, self.title);
        if self.level == LogLevel::Info {
            return write!(f, "{}\x1b[0m\n", to_write);
        }

        if self.cause != String::new() {
            to_write += format!("\n--> {}", self.cause).as_str();
        }

        if self.message != String::new() {
            let mut new_message = String::new();
            for c in self.message.chars() {
                new_message.push(c);
                if c == '\n' {
                    new_message += " | ";
                }
            }

            to_write += format!("\n |\n | {}", new_message).as_str();
        }

        if self.hint != String::new() {
            to_write += format!("\n |\n\x1b[1;34m | ? {}", self.hint).as_str();
        }
        write!(f, "{}\n\x1b[0m\n", to_write)
    }
}

#[derive(Clone)]
pub struct Logger {
    logs: Vec<Log>,
}

impl Logger {
    pub fn new() -> Self {
        Logger { logs: vec![] }
    }

    /// Add a `Log` object to the log list without checking
    pub fn add_log(&mut self, log: Log) {
        self.logs.push(log);
    }

    /// When it's time to reveal all the stocked logs to the user \
    /// Print each contained logs in the log list \
    /// When the logger is set as a killer, the program is stopped after
    /// having printed all logs
    ///
    /// NOTE The function is super simple because each log has a way to be
    /// printed, implemented from `fmt::Display`
    pub fn print_all(&self) {
        let mut is_killer = false;

        for log in &self.logs {
            if log.level == LogLevel::Error {
                is_killer = true;
            }
            print!("{}", log);
        }

        if is_killer {
            process::exit(1);
        }
    }

    /// Designed to be used in the others functions (which need to return a
    /// specific result with the Logger or with nothing) \
    /// ```
    /// fn foo() -> Result<(), Logger> {
    ///     let mut logger = Logger::new();
    ///     ...     
    ///
    ///     logger.get_result()
    /// }
    /// ```
    #[allow(unused)]
    pub fn get_result(&self) -> Result<(), Self> {
        self.get_result_with_value::<()>(())
    }

    /// Same as `Self::get_result()` but for functions which need to return a
    /// specific result with the Logger or with a specific value \
    /// ```
    /// fn foo() -> Result<i32, Logger> {
    ///     let mut logger = Logger::new();
    ///     ...
    ///
    ///     logger.get_result_with_value::<i32>(1)
    /// }
    /// ```
    #[allow(unused)]
    pub fn get_result_with_value<T>(&self, value: T) -> Result<T, Self> {
        if self.logs.len() == 0 {
            Err(self.clone())
        } else {
            Ok(value)
        }
    }

    /// When it's useless to return a `Result` structure with
    /// `Self::get_result()`
    #[allow(unused)]
    pub fn interpret(&self) {
        if self.logs.len() != 0 {
            self.print_all();
        }
    }
}

/// Transform a line of tokens to a printable string for a log
pub fn line_to_string(line: &Vec<Token>, token_i: usize) -> String {
    let mut result = String::from("\t");
    
    let mut i = 0;
    for token in line {
        if token_i > 0 && i == token_i - 1 {
            result += &"\x1b[31m";
        }
        result += &format!("{}\x1b[0m ", tokens::token_to_string(token));
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

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fmt;

use crate::level::LogLevel;

#[derive(Clone, PartialEq, Eq)]
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

    pub fn level(&self) -> &LogLevel {
        &self.level
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

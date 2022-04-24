// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
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

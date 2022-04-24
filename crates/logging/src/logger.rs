// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fmt;
use std::process;

use jup::tokens::Token;

use crate::level::LogLevel;
use crate::log::Log;

#[derive(Clone, PartialEq, Eq)]
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
    /// The program is stopped after having printed all logs but the parameter 
    /// `exit_if_error` can override that, if it's set as true, the program will be 
    /// killed if the logs are errors, else the program isn't killed
    /// even if logs are errors. \
    /// But if the logs contain errors, the function will return "true"
    ///
    /// NOTE The function is super simple because each log has a way to be
    /// printed, implemented from `fmt::Display`
    pub fn print_all(&self, exit_if_error: bool) -> bool {
        let mut is_killer = false;

        for log in &self.logs {
            if log.level() == &LogLevel::Error {
                is_killer = true;
            }
            print!("{}", log);
        }

        if exit_if_error && is_killer {
            process::exit(1);
        }

        is_killer
    }

    /// Designed to be used in the others functions (which need to return a
    /// specific result with the Logger or with nothing) \
    /// Return if the logger contains error logs or not \
    /// ```
    /// fn foo() -> Result<(), Logger> {
    ///     let mut logger = Logger::new();
    ///     ...     
    ///
    ///     logger.get_result()
    /// }
    /// ```
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
        for log in &self.logs {
            if log.level() == &LogLevel::Error {
                return Err(self.clone());
            }
        } // else
        Ok(value)
    }

    /// When it's useless to return a `Result` structure with
    /// `Self::get_result()`
    pub fn interpret(&self) {
        if self.logs.len() != 0 {
            self.print_all(true);
        }
    }
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::env;

use rslog::{
    level::LogLevel, 
    log::Log, 
    logger::Logger
};

const HELP_HINT: &str = "Run again, with the option flag '-h' to get the help page";

/// List of the possible options
const OPTION_FLAGS: [char; 6] = [
    'h', // help
    'l', // library
    'p', // platform
    'o', // output
    'd', // directory
    'a', // add
];

pub struct Args {
    sys_args: Vec<String>,

    sources: Vec<String>,
    options: Dict<String, String>,

    /// Note : It's an argument starting by '-'
    previous_is_option: bool,
    /// "If the check1 is passed" 
    check_step_1: bool, 
}

impl Args {
    pub fn new() -> Self {
        Args {
            sys_args: env::args().collect(),

            sources: vec![],
            options: Dict::new(),

            previous_is_option: false,
            check_step_1: false,
        }
    }

    /// Main function that must be called by the user of this structure
    pub fn run(&mut self) {
        self.check();
        self.parse();
    }

    /// Checks if the passed arguments in the command line arguments are all
    /// valid : so if each option has a value, if the strings are composed
    /// by normal characters only, ...
    fn check(&mut self) {
        let mut logger = Logger::new();

        // When the first check is not passed, does it and returns to permit to
        // call `self.parse()`
        if !self.check_step_1 {
            if self.sys_args.len() == 1 {
                logger.add_log(
                    Log::new(
                        LogLevel::Error,
                        "No given arguments".to_string(),
                        String::new(),
                    )
                    .add_hint(HELP_HINT.to_string()),
                );
            }

            logger.interpret();

            self.check_step_1 = true;
            return;
        }

        // Here, the first check is passed and `self.parse()` was called

        // Checks for the options
        for (option_flag, _option_value) in &self.options {
            // Checks if the option flag is contained into `OPTIONS` too
            let mut is_valid = false;
            for flag in OPTION_FLAGS {
                // Index 1 is the letter
                // "-h", index 0 is "-" and index 1 is "h"
                // `unwrap()` is called because here is no option's flag
                // different than this writing: "-x": no error can be raised
                if option_flag.chars().nth(1).unwrap() == flag {
                    is_valid = true;
                    break; // yes it is, I stop the check
                }
            }

            // The flag was not found in `OPTIONS`
            if !is_valid {
                logger.add_log(
                    Log::new(
                        LogLevel::Warning,
                        "Ignored option flag".to_string(),
                        format!(
                            "The given option flag '{}' does not match with any valid flag",
                            option_flag
                        ),
                    )
                    .add_hint(HELP_HINT.to_string()),
                );
            }
        }

        logger.interpret()
    }

    /// Initializes `self.options` with all options in the command line
    /// arguments as a `Dict`
    ///
    /// Example: `[{"-o": "bin/prog"}, {"-l": "my_lib.so"}]`
    ///
    /// Initializes `self.sources` with all arguments which are not
    /// an option (flag or value)
    ///
    /// Example: `["src/main.ju", "foo.ju"]`
    fn parse(&mut self) {
        let mut key = String::new(); // option flag

        for arg in self.sys_args.clone() {
            // If the option has an empty value, it will be inserted anyway
            if self.is_option(&arg) && self.previous_is_option {
                key = arg;
                self.options.insert(key.clone(), String::new());
                continue;
            }

            // The option has a value
            if self.is_option(&arg) {
                // See : `self.is_option()`
                key = arg;
                continue;
            }

            // The current argument is the option value
            if self.previous_is_option {
                self.previous_is_option = false;
                // so, it's possible to push a new option {flag: value}
                self.options.insert(key.clone(), arg.clone());
                continue;
            }

            // It's not an option, so it's a source file
            self.sources.push(arg.clone());
        }

        // Removes the first argument of `sources` because it's the binary path
        self.sources = self.sources[1..].to_vec();
        self.check(); // do the second check
    }

    /// Easy way to know if an argument is an option flag 
    ///
    /// Updates the value of `self.previous_is_option`
    ///
    /// Note : An argument starting by the '-' character is an option flag
    /// and the next argument is an option value.
    fn is_option(&mut self, arg: &String) -> bool {
        // an argument cannot be null, that why `unwrap()` is called
        if arg.chars().nth(0).unwrap() == '-' {
            self.previous_is_option = true;
            true
        } else {
            false
        }
    }

    pub fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }

    pub fn get_options(&self) -> &Dict<String, String> {
        &self.options
    }

    /// Associated function to call to manage a flag when exists
    pub fn when_flag<F: FnMut(String)>(flag: char, options: &Dict<String, String>, mut do_what: F) {
        match options.get(format!("-{}", flag).as_str()) {
            Some(value) => do_what(value.to_string()),
            None => {}
        }
    }
}

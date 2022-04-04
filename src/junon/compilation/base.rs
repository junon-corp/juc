// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::fs::File;
use std::io::Write;

use crate::junon::{
    args::Args,
    compilation::{
        data::CompilerData,
        defaults,
        linux::LinuxCompiler,
        objects::{
            function::Function, 
            variable::Variable
        },
        parsing::{
            parser::Parser, 
            tokens::*
        },
        caller::Caller,
        scope::Scope,
    },
    logger::*,
    platform, platform::Platform,
};

/// Run the right compiler according to the platform and set some important
/// parameters as a `CompilerData` object sent to the platform's compiler
pub fn run_compiler(sources: &Vec<String>, options: &Dict<String, String>) {
    let mut logger = Logger::new();

    let mut is_library: bool = false;
    Args::when_flag('l', options, |_| {
        is_library = true;
        logger.add_log(Log::info("Library building".to_string()));
    });

    let mut platform: Platform = platform::get_current();
    Args::when_flag('p', options, |mut platform_id: String| {
        platform_id = platform_id.to_lowercase();
        platform = platform::get_from_id(platform_id)
    });

    // Raise an error before printing the log saying the platform
    match platform.clone() {
        Platform::Unknown(invalid_platform_id) => {
            logger.add_log(
                Log::new(
                    LogLevel::Error,
                    "Invalid platform".to_string(),
                    format!(
                        "Platform '{}' is not compatible with the current \
                    version of 'juc'",
                        invalid_platform_id
                    ),
                )
                .add_hint(format!(
                    "Available platforms : {}",
                    platform::AVAILABLE_PLATFORMS
                )),
            );
        }
        _ => {} // valid platform
    }
    logger.interpret();

    logger.add_log(Log::info(format!("Platform : '{:?}'", platform)));
    logger.interpret();

    // Set important information for the compiler
    let data = CompilerData {
        is_library,

        sources: sources.clone(),
        options: options.clone(),

        stream: None,
        parser: None,
        
        current_scope: Scope::new(),
        current_line: vec![],
        current_token: Token::None,

        variable_stack: Dict::new(),
        i_variable_stack: 0,
    };

    // Run the right compiler according to the platform
    match platform {
        Platform::Android => {
            todo!()
        }
        Platform::IOS => {
            todo!()
        }
        Platform::Linux => LinuxCompiler::new(data).run(),
        Platform::MacOS => {
            todo!()
        }
        Platform::Windows => {
            todo!()
        }
        Platform::Unknown(_platform) => panic!() // never happens
    }
}

/// Trait for a Compiler followed by all platform's compilers \
/// Some functions are already defined because they are cross-platform \
/// The general documentation is written here to avoid to write the same
/// documentation to each platform's compilers. But a specific compiler can
/// have its own documentation
pub trait Compiler: Caller {
    /// Starting point \
    /// Do some stuff useful
    fn init(&mut self);

    /// Starting point for each source file
    fn init_one(&mut self, source: &String) {
        let path: String = format!("{}/{}.asm", defaults::BUILD_FOLDER, source);
        
        self.data().stream = Some(File::create(path).unwrap());
        self.data().parser = Some(Parser::new(source));

        self.data().parser.as_mut()
            .unwrap()
            .run();
    }

    /// Main function where each source file is transformed to an objet file
    fn run(&mut self) {
        self.init();

        for source in self.data().sources.clone() {
            // Module name it's the filename without the ".ju" extension
            self.data().current_scope = Scope::from(vec![
                format!("{}", source)
                    .split(defaults::EXTENSION_COMPLETE)
                    .collect::<String>()
            ]);

            self.init_one(&source);
            self.call();
            self.finish_one(&source);
        }

        self.link();
        self.finish();
    }

    /// Methods caller according to the current token
    fn call(&mut self) {
        let parsed: Vec<Vec<Token>> = match &self.data().parser {
            Some(parser) => parser.parsed().clone(),
            None => panic!() // never happens
        };

        for line in parsed.iter() {
            self.data().current_line = line.clone();

            let mut previous_token = Token::None;
            let mut break_line = false; // to break the loop from the closure

            for token in line.iter() {
                if break_line {
                    break;
                }

                self.data().current_token = token.clone();
                self.check_for_instruction(
                    line, 
                    &mut break_line, 
                    &token,
                    &mut previous_token
                );
                previous_token = token.clone();
            }
        }
    }

    /// Function "linked" with `call()` because it does the `Caller` calls
    fn check_for_instruction(
        &mut self, 
        line: &Vec<Token>, 
        break_line: &mut bool,
        token: &Token,
        previous_token: &mut Token
    ) {
        let mut line_iter_for_next_tokens = line.iter();
        line_iter_for_next_tokens.next();

        let next_tokens: Vec<Token> = line_iter_for_next_tokens
            .map(| x | x.clone() )
            .collect(); // as vector
            
        // NOTE "break" instructions means : stop reading the line
        match previous_token {
            Token::AssemblyCode => {
                self.when_assembly_code(next_tokens);
                *break_line = true;
                return;
            }
            Token::Assign => self.when_assign(line.to_vec()),
            Token::Function => self.when_function(next_tokens),
            Token::Return => self.when_return(next_tokens),
            Token::Static => {
                self.when_static(next_tokens);
                *break_line = true;
                return;
            }
            Token::Variable => {
                self.when_variable(next_tokens);
                *break_line = true;
                return;
            }

            // First token of the line
            Token::None => {
                // Lonely token, execute it right now
                if line.len() == 1 {
                    *previous_token = token.clone();
                    
                    // Call again with same arguments
                    self.check_for_instruction(
                        line, 
                        break_line,
                        token,
                        previous_token
                    );
                }
            },
            _ => self.when_other(),
        }
    }

    /// Link all generated files to one output file (library or binary according
    /// to the selected one)
    fn link(&mut self);

    /// Exit point \
    /// Delete all temporary files and do linking
    fn finish(&mut self);

    /// Exit point for each source file
    fn finish_one(&mut self, source: &String);

    /// Data getter
    fn data(&mut self) -> &mut CompilerData;

    // --- ASM code generators

    /// Variable declaration, can 
    fn add_variable(&mut self, variable: Variable);
    fn add_static_variable(&mut self, variable: Variable);
    fn add_function(&mut self, function: Function);

    fn change_variable_value(&mut self, variable: &Variable);

    fn return_(&mut self, value: String);

    /// Directly write some ASM code into current stream file
    fn write_asm(&mut self, to_write: String) {
        write!(
            self.data().stream.as_mut().unwrap(), 
            "{}\n", 
            to_write
        ).unwrap();
    }
}

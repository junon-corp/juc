// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::io::Write;

use crate::junon::{
    compilation::{
        objects::{
            function::Function,
            variable::Variable,
        },
        data::CompilerData,
        linux::LinuxCompiler,
    },
    args::Args,
    logger::*,
    platform, platform::Platform,
};

/// Run the right compiler according to the platform and set some important
/// parameters as a `CompilerData` object sent to the platform's compiler
pub fn run_compiler(sources: &Vec<String>, options: &Dict<String, String>) {
    let mut logger = Logger::new();

    let mut is_library: bool = false;
    Args::when_flag('l', options, | _ | {
        is_library = true;
        logger.add_log(
            Log::info("Library building".to_string())
        );    
    });

    let mut platform: Platform = platform::get_current();
    Args::when_flag('p', options, | mut platform_id: String | {
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
                    format!("Platform '{}' is not compatible with the current \
                    version of 'juc'", invalid_platform_id)
                )
                .add_hint(format!("Available platforms : {}", 
                    platform::AVAILABLE_PLATFORMS
                ))
            );
        },
        _ => {} // valid platform
    }
    logger.interpret();

    logger.add_log(
        Log::info(format!("Platform : '{:?}'", platform))
    );
    logger.interpret();

    // Set important information for the compiler
    let data = CompilerData {
        sources: sources.clone(),
        options: options.clone(),
        is_library,
        stream: None,
    };

    // Run the right compiler according to the platform
    match platform {
        Platform::Android => { todo!() },
        Platform::IOS => { todo!() },
        Platform::Linux => { LinuxCompiler::new(data).run() },
        Platform::MacOS => { todo!() },
        Platform::Windows => { todo!() },
        _ => panic!(), // never happens
    }
}

/// Trait for a Compiler followed by all platform's compilers \
/// Some functions are already defined because they are cross-platform \
/// The general documentation is written here to avoid to write the same
/// documentation to each platform's compilers. But a specific compiler can
/// have its own documentation
pub trait Compiler {
    /// Starting point \
    /// Do some stuff useful 
    fn init(&mut self);

    /// Main function where each source file is transformed to an objet file
    fn run(&mut self);

    /// Link all generated files to one output file (library or binary according
    /// to the selected one)
    fn link(&mut self);

    /// Exit point \
    /// Delete all temporary files and do linking
    fn finish(&mut self);
    
    /// Data getter
    fn data(&mut self) -> &mut CompilerData; 

    // --- ASM code generators
    
    fn add_variable(&mut self, variable: Variable);
    fn add_function(&mut self, function: Function);
    // fn add_structure(&mut self, structure: Structure);

    /// Directly write some ASM code
    fn write_asm(&mut self, asm_code: String) {
        match &mut self.data().stream {
            Some(stream) => write!(stream, "{}\n", asm_code).unwrap(),
            _ => panic!(), // never happens
        }
    }
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod base;
pub mod caller;
pub mod data;
pub mod defaults;
pub mod linux;
pub mod scope;

use std::collections::HashMap as Dict;

use jup::tokens::Token;

use args::Args;

use logging::{
    level::LogLevel,
    log::Log,
    logger::Logger,
};

use platform::Platform;

use crate::base::Compiler;
use crate::data::CompilerData;
use crate::linux::LinuxCompiler;
use crate::scope::Scope;

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

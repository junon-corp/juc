// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod compilers;
pub mod data;
pub mod defaults;
pub mod scope;

use std::collections::HashMap as Dict;

use jup::lang::{
    tokens::Token,
    elements::Element,
};

use x64asm::formatter::Formatter;
use args::Args;
use rslog::{
    level::LogLevel, 
    log::Log, 
    logger::Logger
};
use platform::Platform;

use crate::{
    compilers::base::Compiler,
    compilers::linux::LinuxCompiler,
    data::CompilerData,
    scope::Scope,
};

/// Runs the right compiler according to the platform and set some important
/// parameters as a `CompilerData` object sent to the platform's compiler
pub fn run_compiler(sources: &Vec<String>, options: &Dict<String, String>) {
    let mut logger = Logger::new();

    // Retrieves the output mode from `Args`
    let mut is_library: bool = false;
    Args::when_flag('l', options, |_| {
        is_library = true;
        logger.add_log(Log::info("Library building".to_string()));
    });

    // Retrieves the platform from `Args`
    let mut platform: Platform = platform::get_current();
    Args::when_flag('p', options, |mut platform_id: String| {
        platform_id = platform_id.to_lowercase();
        platform = platform::get_from_id(platform_id)
    });

    // Tells the current platform. It can be wrong (checked above)
    logger.add_log(Log::info(format!("Platform : '{:?}'", platform)));

    // Platform checking for wrong not compatible platforms
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

    // Sets important information for the compiler
    let data = CompilerData {
        is_library,

        sources: sources.clone(),
        options: options.clone(),

        asm_formatter: Formatter::new(false),

        current_source: String::new(),
        current_parsed: vec![],
        current_scope: Scope::new(),

        next_element: Element::Other(Token::None),
        is_skip_next: false,

        variable_stack: Dict::new(),
        i_variable_stack: 0,
    };

    // Runs the right compiler according to the platform
    match platform {
        Platform::Android => todo!(),
        Platform::IOS => todo!(),
        Platform::Linux => LinuxCompiler::new(data).run(),
        Platform::MacOS => todo!(),
        Platform::Windows => todo!(),

        // Already checked previously in this own function
        Platform::Unknown(_platform) => panic!(), // never happens
    }
}

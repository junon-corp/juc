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
use strings::manager::StringsManager;

use crate::{
    compilers::{
        base::Compiler,
        linux::LinuxCompiler,
    },
    data::{
        CompilerData,
        CompilerTools,
        CompilerCodeData,
        CompilerStacksData,
    },
    scope::Scope,
};

/// Runs the right compiler according to the platform and set some important
/// parameters as a `CompilerData` object sent to the platform's compiler
pub fn run_compiler(sources: &Vec<String>, options: &Dict<String, String>, sm: &StringsManager) {
    let mut logger = Logger::new();

    // Retrieves the output mode from `Args`
    let mut is_library: bool = false;
    Args::when_flag('l', options, |_| {
        is_library = true;
        logger.add_log(Log::info(sm.get().logs.infos.library_building.title.as_ref().unwrap().get(sm)));
    });

    // Retrieves the platform from `Args`
    let mut platform: Platform = platform::get_current();
    Args::when_flag('p', options, |mut platform_id: String| {
        platform_id = platform_id.to_lowercase();
        platform = platform::get_from_id(platform_id)
    });

    // Tells the current platform. It can be wrong (checked above)
    logger.add_log(Log::info(
        sm.get().logs.infos.platform.title.as_ref().unwrap().get(sm)
            .replacen("{}", format!("{:?}", platform).as_str(), 1)
    ));

    // Platform checking for wrong not compatible platforms
    match platform.clone() {
        Platform::Unknown(invalid_platform_id) => {
            logger.add_log(
                Log::new(
                    LogLevel::Error,
                    sm.get().logs.errors.platform.title.as_ref().unwrap().get(sm),
                    sm.get().logs.errors.platform.message.as_ref().unwrap().get(sm)
                        .replacen("{}", &invalid_platform_id, 1)
                )
                .add_hint(sm.get().logs.errors.platform.hint.as_ref().unwrap().get(sm)
                    .replacen("{}", platform::AVAILABLE_PLATFORMS, 1)
                ),
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
    };

    let tools = CompilerTools {
        strings_manager: sm.clone(),
        asm_formatter: Formatter::new(false),
    };

    let code_data = CompilerCodeData {
        current_source: String::new(),
        current_parsed: vec![],
        scope: Scope::new(),

        next_element: Element::Other(Token::None),
        is_skip_next: false,
        is_condition: false,
        n_condition: 0,
    };

    let stacks_data = CompilerStacksData {
        variable_stack: Dict::new(),
        i_variable_stack: 0,

        i_parameter_stack: 0,
    };

    let all_data = (data, tools, code_data, stacks_data);

    // Runs the right compiler according to the platform
    match platform {
        Platform::Android => todo!(),
        Platform::IOS => todo!(),
        Platform::Linux => LinuxCompiler::new(all_data).run(),
        Platform::MacOS => todo!(),
        Platform::Windows => todo!(),

        // Already checked previously in this own function
        Platform::Unknown(_platform) => panic!(), // never happens
    }
}

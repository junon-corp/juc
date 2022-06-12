// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::env;
use std::process::Command;

use rslog::{
    level::LogLevel, 
    log::Log, 
    logger::Logger
};

/// Constants where all available platforms are stored, "linked" with `Platform`
///
/// Should be updated in the same of `Platform` updates
pub const AVAILABLE_PLATFORMS: &str = "Android, IOS, Linux, MacOS, Windows";

/// All available platform where code can be compiled.
///
/// Also can store an unknown platform if any right platform was retrieved
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Platform {
    Android,
    IOS,
    Linux,
    MacOS,
    Windows,
    /// Unknown retrieved platform, will stop the program when matched with
    Unknown(String),
}

/// Gets the platform where the compiler is currently running.
/// 
/// See constants at https://doc.rust-lang.org/std/env/consts/constant.OS.html
pub fn get_current() -> Platform {
    // The platform identifier is set as lowercase according to the
    // documentation page (See : This function's documentation)
    let platform_id: &str = env::consts::OS;
    get_from_id(platform_id.to_string())
}

/// Gets the platform as a `Platform` enum object from an identifier.
///
/// The identifier should be as lowercase
pub fn get_from_id(platform_id: String) -> Platform {
    match platform_id.as_str() {
        "android" => Platform::Android,
        "ios" => Platform::IOS,
        "linux" => Platform::Linux,
        "macos" => Platform::MacOS,
        "windows" => Platform::Windows,
        _ => Platform::Unknown(platform_id),
    }
}

/// Way to call a program on the system
///
/// Note : The output is never disabled
pub fn exec(program_id: String, arguments: &[String]) {
    let output = Command::new(program_id.clone())
        .args(arguments)
        .output()
        .unwrap();

    let program_result: String = output.stderr.into_iter().map(|x| x as char).collect();

    if program_result != String::new() {
        let mut logger = Logger::new();
        logger.add_log(
            Log::new(
                LogLevel::Error,
                format!("Execution of '{}' failed", program_id),
                program_result,
            )
            .add_hint(
                "The called program may be not installed. It could be a \
                bug from 'juc' or the called program"
                    .to_string(),
            ),
        );
        logger.interpret();
    }
}

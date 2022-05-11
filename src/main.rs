// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process;

use args::Args;

use compilation::defaults;

use rslog::{level::LogLevel, log::Log, logger::Logger};

fn main() {
    let mut args = Args::new();
    args.run();

    let sources: &Vec<String> = args.get_sources();
    let options: &Dict<String, String> = args.get_options();

    let mut logger = Logger::new();

    Args::when_flag('h', options, |_| help());
    Args::when_flag('d', options, |path: String| {
        let current_dir = Path::new(&path);
        if !current_dir.is_dir() || !current_dir.exists() {
            logger.add_log(Log::new(
                LogLevel::Error,
                "Invalid path OR Not a directory".to_string(),
                format!(
                    "The given directory '{}' does not exist or it's not a directory",
                    path
                ),
            ));
        }
        logger.interpret();

        env::set_current_dir(&current_dir).unwrap();
    });

    logger.add_log(Log::info(format!(
        "Working directory : '{}'",
        env::current_dir().unwrap().display()
    )));

    // Check after current directory set
    for source in sources {
        let path = Path::new(source);
        if !path.exists() {
            logger.add_log(Log::new(
                LogLevel::Error,
                "Source file does not exist".to_string(),
                format!(
                    "The given source file '{}' cannot be found in the current directory",
                    source
                ),
            ));
        }
        if path.extension() != Some(OsStr::new("ju")) {
            let error_message = match path.extension() {
                Some(extension) => {
                    format!(
                        "The given source file '{}' does not have right \
                        extension, it should be '{}' not {:?}",
                        source,
                        defaults::EXTENSION,
                        extension
                    )
                }
                None => {
                    format!(
                        "The given source file '{}' should have an extension",
                        source
                    )
                }
            };

            logger.add_log(
                Log::new(
                    LogLevel::Error,
                    "Invalid extension file".to_string(),
                    error_message,
                )
                .add_hint(format!(
                    "Rename '{}' by '{}.{}'",
                    source,
                    source,
                    defaults::EXTENSION
                )),
            );
        }
    }

    logger.interpret();

    compilation::run_compiler(sources, options);

    let mut logger = Logger::new();
    logger.add_log(Log::info("Finished".to_string()));
    logger.interpret();
}

fn help() {
    let to_write = "Junon help page (command line)\n".to_string()
        + "\n"
        + "juc <?sources> <?options...>\n"
        + "- ?sources : paths of the Junon source code files that you want to compile\n"
        + "- ?options : an option should look like that: -<option flag> <option value>\n"
        + "\n"
        + "Available option flags:\n"
        + "\t-h : Get this help page\n"
        + "\t-l : Make a library instead of a binary\n"
        + "\n"
        + "\t-p <platform name> : Compile for this platform\n"
        + "\t\t(Android, IOS, Linux, MacOS, Windows)\n"
        + "\t-o <path> : Path for the output file\n"
        + "\t-d <path> : Replace the current directory context location\n";

    print!("\x1b[1m{}\x1b[0m", to_write);
    process::exit(0);
}

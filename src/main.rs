// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::env;
use std::path::Path;
use std::process;

mod junon;
use junon::{
    compilation::{
        base::*,
    },
    args::Args, 
    logger::*
};

fn main() {
    let mut args = Args::new();
    args.run();

    let sources: &Vec<String> = args.get_sources();
    let options: &Dict<String, String> = args.get_options();

    let mut logger = Logger::new();

    Args::when_flag('h', options, | _ | help());
    Args::when_flag('d', options, | path: String | {
        let current_dir = Path::new(&path);
        if ! current_dir.is_dir() || ! current_dir.exists() {
            logger.add_log(
                Log::new(
                    LogLevel::Error,
                    "Invalid path OR Not a directory".to_string(),
                    format!("The given directory '{}' does not exist or it's not a directory", path),
                )
            );
        }
        logger.interpret();

        env::set_current_dir(&current_dir).unwrap();
    });

    logger.add_log(
        Log::info(format!("Working directory : '{}'", env::current_dir()
            .unwrap().display())
        )
    );

    logger.interpret();

    run_compiler(sources, options);
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
        + "\t-d <path> : Replace the current directory context location\n"
    ;

    print!("\x1b[1m{}\x1b[0m", to_write);
    process::exit(0);
}

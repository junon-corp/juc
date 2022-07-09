// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::{
    collections::HashMap as Dict,
    env,
    ffi::OsStr,
    path::Path,
    process,
};

use rslog::{
    level::LogLevel, 
    log::Log, 
    logger::Logger
};

use args::Args;
use compilation::defaults;
use strings;
use strings::manager::StringsManager;

/// Retrieves all useful stuffs for the compiler, can set some things from the
/// retrieved options before calling the compiler.
///
/// Checks the given source files for their existence and right file's extension 
///
/// Then, it gives the control to compilation's crate
fn main() {
    let mut args = Args::new();
    args.run();

    let sources: &Vec<String> = args.get_sources();
    let options: &Dict<String, String> = args.get_options();

    let mut logger = Logger::new();

    let mut sm = strings::init_strings();
    Args::when_flag('s', options, | speak_lang | {
        sm.set_speak_language(speak_lang);
    });
    
    Args::when_flag('h', options, |_| help(&sm));
    Args::when_flag('d', options, |path: String| {
        let current_dir = Path::new(&path);
        if !current_dir.is_dir() || !current_dir.exists() {
            logger.add_log(Log::new(
                LogLevel::Error,
                sm.get().logs.errors.invalid_path_or_not_a_directory.title.as_ref().unwrap().get(&sm),
                sm.get().logs.errors.invalid_path_or_not_a_directory.message.as_ref().unwrap().get(&sm)
                    .replacen("{}", &path, 1)
            ));
        }
        logger.interpret();

        env::set_current_dir(&current_dir).unwrap();
    });

    logger.add_log(Log::info(format!(
        "{}{}",
        sm.get().logs.infos.working_directory.title.as_ref().unwrap().get(&sm),
        env::current_dir().unwrap().display()
    )));

    // Check after current directory set
    for source in sources {
        let path = Path::new(source);

        if !path.exists() {
            logger.add_log(Log::new(
                LogLevel::Error,
                sm.get().logs.errors.source_file_does_not_exist.title.as_ref().unwrap().get(&sm),
                sm.get().logs.errors.source_file_does_not_exist.message.as_ref().unwrap().get(&sm)
                    .replacen("{}", &path.to_str().unwrap(), 1)
            ));
        }
        if path.extension() != Some(OsStr::new(defaults::EXTENSION)) {
            let error_message = match path.extension() {
                Some(extension) => {
                    sm.get().logs.errors.wrong_file_extension.title.as_ref().unwrap().get(&sm)
                        .replacen("{}", source, 1)
                        .replacen("{}", defaults::EXTENSION, 1)
                        .replacen("{:?}", &extension.to_os_string().into_string().unwrap(), 1)
                }
                None => {
                    sm.get().logs.errors.no_file_extension.title.as_ref().unwrap().get(&sm)
                        .replacen("{}", source, 1)
                }
            };

            logger.add_log(
                Log::new(
                    LogLevel::Error,
                    sm.get().logs.errors.invalid_file_extension.title.as_ref().unwrap().get(&sm),
                    error_message,
                )
                .add_hint(
                    sm.get().logs.errors.invalid_file_extension.title.as_ref().unwrap().get(&sm)
                        .replacen("{}", source, 2)
                        .replacen("{}", defaults::EXTENSION, 1)
                ),
            );
        }
    }

    logger.interpret();

    // Run the right compiler with retrieved options for each source file
    // All source files will be linked together to one library or binary file
    compilation::run_compiler(sources, options, &sm);

    let mut logger = Logger::new();
    logger.add_log(Log::info(sm.get().logs.infos.finished.title.as_ref().unwrap().get(&sm)));
    logger.interpret();
}

/// Program documentation and usage specifications
///
/// Called when "-h" was found in options
fn help(sm: &StringsManager) {
    let to_write = [
        sm.get().help.title.get(sm),
        "juc <?sources> <?options...>\n".to_string(),
        sm.get().help.arguments.sources.get(sm),
        sm.get().help.arguments.options.get(sm),
        sm.get().help.available_flags.title.get(sm),
        sm.get().help.available_flags.h.get(sm),
        sm.get().help.available_flags.l.get(sm),
        sm.get().help.available_flags.p.get(sm),
        "\t\t(Android, IOS, Linux, MacOS, Windows)\n".to_string(),
        sm.get().help.available_flags.o.get(sm),
        sm.get().help.available_flags.d.get(sm),
        sm.get().help.available_flags.a.get(sm),
        sm.get().help.available_flags.s.get(sm),
    ].join("\n");

    print!("\x1b[1m{}\x1b[0m", to_write);
    process::exit(0);
}

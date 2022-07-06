// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MultiString {
    en: String,
    fr: Option<String>,
    de: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    title: Option<MultiString>,
    message: Option<MultiString>,
    hint: Option<MultiString>,
}

#[derive(Debug, Deserialize)]
pub struct Errors {
    no_given_arguments: Log,
    invalid_path_or_not_a_directory: Log,
    source_file_does_not_exist: Log,
    invalid_file_extension: Log,
    wrong_file_extension: Log,
    no_file_extension: Log,
    execution_failed: Log,
    platform: Log
}

#[derive(Debug, Deserialize)]
pub struct Infos {
    library_building: Log,
    working_directory: Log,
    ignored_option_flag: Log,
    finished: Log,
}    

#[derive(Debug, Deserialize)]
pub struct Logs {
    infos: Infos,
    errors: Errors,
}

#[derive(Debug, Deserialize)]
pub struct HelpAvailableFlags {
    title: MultiString,
    h: MultiString,
    l: MultiString,
    p: MultiString,
    o: MultiString,
    d: MultiString,
    a: MultiString,
}

#[derive(Debug, Deserialize)]
pub struct HelpArguments {
    sources: MultiString,
    options: MultiString,
}

#[derive(Debug, Deserialize)]
pub struct Help {
    title: MultiString,
    arguments: HelpArguments,
    available_flags: HelpAvailableFlags
}

#[derive(Debug, Deserialize)]
pub struct Strings {
    help: Help,
    logs: Logs,
}

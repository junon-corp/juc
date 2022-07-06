// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MultiString {
    en: String,
    fr: String,
    // probably other languages later
}

#[derive(Deserialize)]
pub struct Log {
    title: MultiString,
    message: MultiString,
    hint: MultiString,
}

#[derive(Deserialize)]
pub struct Errors {
    no_given_arguments: MultiString,
    invalid_path_or_not_a_directory: MultiString,
    source_file_does_not_exist: MultiString,
    invalid_file_extension: MultiString,
    wrong_file_extension: MultiString,
    no_file_extension: MultiString,
    execution_failed: MultiString,
    platform: MultiString
}

#[derive(Deserialize)]
pub struct Infos {
    library_building: MultiString,
    working_directory: MultiString,
    ignored_option_flag: MultiString,
    finished: MultiString,
}    

#[derive(Deserialize)]
pub struct Logs {
    infos: Infos,
    errors: Errors,
}

#[derive(Deserialize)]
pub struct HelpAvailableFlags {
    title: MultiString,
    h: MultiString,
    l: MultiString,
    p: MultiString,
    o: MultiString,
    d: MultiString,
    a: MultiString,
}

#[derive(Deserialize)]
pub struct HelpArguments {
    sources: MultiString,
    options: MultiString,
}

#[derive(Deserialize)]
pub struct Help {
    title: MultiString,
    arguments: HelpArguments,
    available_flags: HelpAvailableFlags
}

#[derive(Deserialize)]
pub struct Strings {
    help: Help,
    logs: Logs,
}

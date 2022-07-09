// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MultiString {
    pub en: String,
    pub fr: Option<String>,
    pub de: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Log {
    pub title: Option<MultiString>,
    pub message: Option<MultiString>,
    pub hint: Option<MultiString>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Errors {
    pub no_given_arguments: Log,
    pub invalid_path_or_not_a_directory: Log,
    pub source_file_does_not_exist: Log,
    pub invalid_file_extension: Log,
    pub wrong_file_extension: Log,
    pub no_file_extension: Log,
    pub execution_failed: Log,
    pub platform: Log
}

#[derive(Clone, Debug, Deserialize)]
pub struct Infos {
    pub library_building: Log,
    pub working_directory: Log,
    pub ignored_option_flag: Log,
    pub finished: Log,
}    

#[derive(Clone, Debug, Deserialize)]
pub struct Logs {
    pub infos: Infos,
    pub errors: Errors,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HelpAvailableFlags {
    pub title: MultiString,
    pub h: MultiString,
    pub l: MultiString,
    pub p: MultiString,
    pub o: MultiString,
    pub d: MultiString,
    pub a: MultiString,
    pub s: MultiString,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HelpArguments {
    pub sources: MultiString,
    pub options: MultiString,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Help {
    pub title: MultiString,
    pub arguments: HelpArguments,
    pub available_flags: HelpAvailableFlags
}

#[derive(Clone, Debug, Deserialize)]
pub struct Strings {
    pub help: Help,
    pub logs: Logs,
}

// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;

mod junon;
use junon::{args::Args, logger::*};

fn main() {
    let mut args = Args::new();
    args.run();

    let sources: &Vec<String> = args.get_sources();
    let options: &Dict<String, String> = args.get_options();

    Args::when_flag('h', options, |value| help());
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
}

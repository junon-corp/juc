// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;

use crate::junon::{
    args::Args,
    platform,
    platform::Platform,
};

pub fn run_compiler(sources: &Vec<String>, options: &Dict<String, String>) {
    let mut is_library: bool = false;
    Args::when_flag('l', options, | _ | {
        is_library = true;
    });

    let mut platform: Platform = platform::get_current();
    Args::when_flag('p', options, | mut platform_id: String | {
        platform_id = platform_id.to_lowercase();
        platform = platform::get_from_id(platform_id)
    });

    println!("Current platform: {:?}", platform);
}

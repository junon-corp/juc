// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;

mod junon;
use junon::{
    args::Args,
    logger::*,
};

fn main() {
    let mut args = Args::new();
    args.parse();

    let sources: &Vec<String> = args.get_sources();
    let options: &Dict<String, String> = args.get_options();
}

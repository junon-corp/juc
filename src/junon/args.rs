// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::collections::HashMap as Dict;
use std::env;

use crate::junon::{
    logger::*,
};

pub struct Args {
    sys_args: Vec<String>,

    sources: Vec<String>,
    options: Dict<String, String>,
}

impl Args {
    pub fn new() -> Self {
        Args {
            sys_args: env::args().collect(),
            sources: vec!(),
            options: Dict::new(),
        }
    }
    
    /// Main function of this structure \
    /// Parse the system argument list to `self.sources` and `self.options`
    pub fn parse(&mut self) {        
        let mut logger = Logger::new(true);

        for sys_arg in self.sys_args.clone() {

        }

        logger.interpret()
    }


    pub fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }

    pub fn get_options(&self) -> &Dict<String, String> {
        &self.options
    }
}

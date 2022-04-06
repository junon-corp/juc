// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use crate::junon::compilation::{
    checking::data::CheckerData,
    parsing::tokens::Token,
};

pub trait Checker {
    /// Entry point with useful stuff did here
    fn init(&mut self);

    fn run(&mut self) {
        self.init();
        self.check();
        self.finish();
    }

    fn check(&mut self);
    fn check_for_instruction(
        &mut self, 
        line: &Vec<Token>, 
        break_line: &mut bool,
        token: &Token,
        previous_token: &mut Token
    );

    /// Exit point with useful stuff did here
    fn finish(&mut self);

    /// Data getter
    fn data(&mut self) -> &mut CheckerData;
}
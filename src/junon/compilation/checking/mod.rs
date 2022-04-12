// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod base;
pub mod data;
pub mod syntax;

use crate::junon::{
    compilation::checking::{
        base::Checker,
        data::CheckerData,
        syntax::SyntaxChecker,
    },
    logger::Logger,
};

pub fn run_checkers(data: CheckerData) -> Result<(), Logger> {
    let mut syntax_checker = SyntaxChecker::new(data);
    syntax_checker.run()?;

    Ok(())
}

// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

use jup::{
    checking::syntax::SyntaxChecker,
    parser::Parser, 
    lang::tokens::Token,
};

use rslog::{
    *,
    level::LogLevel,
    log::Log,
    logger::Logger,
};

use crate::{
    caller::Caller,
    data::CompilerData,
    defaults,
    scope::Scope,
};

use objects::{
    function::Function, 
    variable::Variable
};

/// Trait for a Compiler followed by all platform's compilers \
/// Some functions are already defined because they are cross-platform \
/// The general documentation is written here to avoid to write the same
/// documentation to each platform's compilers. But a specific compiler can
/// have its own documentation
pub trait Compiler<'a>: Caller<'a> {
    /// Starting point \
    /// Do some stuff useful
    fn init(&mut self);

    /// Starting point for each source file
    fn init_one(&mut self, source: &String) {
        self.data().current_source = format!(
            "{}/{}.asm", 
            defaults::BUILD_FOLDER, 
            source
        );
        
        self.data().parser = Some(
            Parser::from_path(Path::new(source)).unwrap()
        );
        self.data().parser.as_mut().unwrap().run();

        // NOTE : `Parser::parsed()` returns `&Vec<Token>`
        // SEE : https://github.com/junon-corp/jup/blob/main/src/parser.rs
        
        self.data().current_parsed = &self.data().parser.as_ref().unwrap().parsed().clone();


        // Run syntax checker for the current source file
        let mut checker = SyntaxChecker::new(source, self.data().current_parsed);
        checker.run();
    }

    /// Main function where each source file is transformed to an objet file
    fn run(&mut self) {
        self.init();

        // NOTE : If any syntax problem is found during syntax checking, the 
        // program will be terminated. They should be retrieved and printed 
        // after all sources checks
        // TODO : Update "jup" according to the previous NOTE
        for source in self.data().sources.clone() {
            // Module name is the filename without the ".ju" extension
            self.data().current_scope = Scope::from(vec![
                format!("{}", source)
                    .split(defaults::EXTENSION_COMPLETE)
                    .collect::<String>()
            ]);

            self.init_one(&source);
            self.call();
            self.finish_one(&source);
        }

        self.link();
        self.finish();
    }

    /// Methods caller according to the current token
    fn call(&mut self) {
        // If tokens have to be skipped + how much to skip?
        let mut skip_mode: (bool, usize) = (false, 0);

        for token in self.data().current_parsed.iter() {
            // Skip until all asked tokens for skipping while be skipped
            if skip_mode.0 {
                skip_mode.1 -= 1;
                continue;
            }

            self.data().current_token = &token;

            let to_skip: usize = self.check();
            skip_mode = (if to_skip > 0 { true } else { false }, to_skip);
        }
    }

    /// Returns how much tokens should be skip (how much token was read because
    /// of another but not checked by this function)
    fn check(&mut self) -> usize {
        // `when...` always return how much tokens they read to skip them
        match self.data().current_token {
            Token::Assembly => self.when_assembly_code(),
            Token::Assign => self.when_assign(),
            Token::Function => self.when_function(),
            Token::Return => self.when_return(),
            Token::Static => self.when_static(),
            Token::Variable => self.when_variable(),

            Token::Print => self.when_print(),
            Token::Exit => self.when_exit(),
            _ => self.when_other(),
        }
    }

    /// Link all generated files to one output file (library or binary according
    /// to the selected one)
    fn link(&mut self);

    /// Exit point \
    /// Delete all temporary files and do linking
    fn finish(&mut self) {}

    /// Exit point for each source file
    fn finish_one(&mut self, source: &String);

    /// Data getter
    fn data(&mut self) -> &mut CompilerData<'a>;

    // --- ASM code generators

    /// Variable declaration, can 
    fn add_variable(&mut self, variable: Variable);
    fn add_static_variable(&mut self, variable: Variable);
    fn add_function(&mut self, function: Function);

    fn change_variable_value(&mut self, variable: &Variable);

    fn return_(&mut self, value: String);

    fn print(&mut self, to_print: String);
    fn exit(&mut self, value: String);
}

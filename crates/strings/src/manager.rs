// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::{
    fs,
    path::Path,
};

use crate::structured::*;

pub struct StringsManager {
    deserialized: Strings,
}

impl StringsManager {
    pub fn new(deserialized: Strings) -> Self {
        Self {
            deserialized
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let json_content = fs::read_to_string(path).unwrap();
        Self::new(serde_json::from_str(&json_content).unwrap())
    }

    pub fn from_string(string: &str) -> Self {
        Self::new(serde_json::from_str(string).unwrap())
    }

    pub fn get(&self) -> &Strings {
        &self.deserialized
    }
}

#[test]
fn strings_manager() {
    let path = Path::new("../../src/strings.json");
    let strings = StringsManager::from_path(&path);
    println!("{:#?}", strings.get().logs.errors.no_given_arguments.title);
}

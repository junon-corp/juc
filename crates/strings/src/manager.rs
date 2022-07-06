// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::{
    fs,
    path::Path,
};

use crate::structured::Strings;

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
        
        Self {
            deserialized: serde_json::from_str(&json_content).unwrap()
        }
    }

    pub fn strings(&self) -> &Strings {
        &self.deserialized
    }
}

#[test]
fn strings_manager() {
    let path = Path::new("../../src/strings.json");
    let strings = StringsManager::from_path(&path);
    println!("{:#?}", strings.strings());
}

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
    speak_lang: String,
}

impl StringsManager {
    pub fn new(deserialized: Strings) -> Self {
        Self {
            deserialized,
            speak_lang: "en".to_string(),
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let json_content = fs::read_to_string(path).unwrap();
        Self::new(serde_json::from_str(&json_content).unwrap())
    }

    pub fn from_string(string: &str) -> Self {
        Self::new(serde_json::from_str(string).unwrap())
    }

    pub fn set_speak_language(&mut self, speak_lang: String) {
        self.speak_lang = speak_lang;
    }

    pub fn get(&self) -> &Strings {
        &self.deserialized
    }

    pub fn speak_lang(&self) -> &String {
        &self.speak_lang
    }
}

impl MultiString {
    pub fn get(&self, sm: &StringsManager) -> String {
        let r = match sm.speak_lang().as_str() {
            "en" => Some(self.en.clone()),
            "fr" => self.fr.clone(),
            "de" => self.de.clone(),
            _ => panic!(),
        };

        match r {
            None => self.en.clone(),
            _ => r.unwrap(),
        }
    } 
}

#[test]
fn strings_manager() {
    let path = Path::new("../../src/strings.json");
    let strings = StringsManager::from_path(&path);
    println!("{:#?}", strings.get().logs.errors.no_given_arguments.title);
}

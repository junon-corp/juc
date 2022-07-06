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

/// Gets string in the current language
#[macro_export]
macro_rules! get_string {
    ($strings_manager:expr, $object:expr) => {
        {
            let string_current = match $strings_manager.speak_lang().as_str() {
                "en" => Some($object.en.clone()),
                "fr" => $object.fr.clone(),
                "de" => $object.de.clone(),
                _ => panic!(),
            };
    
            if string_current == None {
                $object.en.clone()
            } else {
                string_current.unwrap()
            }  
        }
    };
}

#[test]
fn strings_manager() {
    let path = Path::new("../../src/strings.json");
    let strings = StringsManager::from_path(&path);
    println!("{:#?}", strings.get().logs.errors.no_given_arguments.title);
}

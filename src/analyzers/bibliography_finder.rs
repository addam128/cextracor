use json::{JsonValue};
use fancy_regex::Regex;

use std::collections::HashMap;

use crate::utils;

use super::traits::Analyzer;

pub(crate) struct BibliographyFinder {

    _bibliography_entry_regex: Regex,
    _bibliography_start_regex: Regex,
    _bibliography_start_found: bool,
    _found: HashMap< String, String >,
    _buffer: String // maybe unused

}

impl BibliographyFinder {

    pub(crate) fn new() -> Result<Self, utils::Error> {

        let key_map = HashMap::new();
        Ok(
            Self {
                _bibliography_entry_regex: Regex::new(r"(?s)\s*(\[[a-zA-Z0-9_\-\#\s]*\])(.*?(?=\n{2}|\[))")?,
                _bibliography_start_regex: Regex::new(r"(?i).*(bibliography|referenced literature|references|literature)\n")?,
                _bibliography_start_found: false,
                _found: key_map,
                _buffer: String::new(),
            }
        )
    }
}

impl Analyzer for BibliographyFinder {

    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {
        let mut to_process = chunk;
        if !self._bibliography_start_found {
            let bibliography_start = self._bibliography_start_regex.find(chunk);
            if bibliography_start.is_ok() {
                let match_option = bibliography_start.unwrap();
                if match_option.is_some() {
                    self._bibliography_start_found = true;
                    // only process from matched bibliography in the current chunk
                    let m = match_option.unwrap();
                    to_process= &chunk[m.end()..]
                }
            }
        }

        if self._bibliography_start_found {
            let bibliography_entries = self._bibliography_entry_regex.captures_iter(to_process);
            for bibliography_entry in bibliography_entries {
                let unwrapped = bibliography_entry?; // this needs to be handled 


                let key = unwrapped.get(1).unwrap().as_str();
                let value = unwrapped.get(2).unwrap().as_str().trim().replace("\n", " ");

                let re = regex::Regex::new(r"\s+").unwrap();
                let formatted_value = re.replace_all(&value, " ");

                if !self._found.contains_key(key) {
                    self._found.insert(String::from(key), (&formatted_value).parse().unwrap());
                }
            }
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<json::JsonValue, utils::Error> {
        let map  = self._found.clone();
        Ok(JsonValue::from(map))
    }

    fn clear(&mut self) -> () {
        self._buffer.clear();
        for set in self._found.values_mut() {
            set.clear();
        }
    }
}

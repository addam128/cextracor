use json::{JsonValue};
use fancy_regex::Regex;

use std::collections::HashMap;

use crate::utils;

use super::traits::Analyzer;

pub(crate) struct BibliographyFinder {

    _bibliography_entry_regex: Regex,
    _bibliography_start_regex: Regex,
    _bibliography_start_found: bool,
    _whitespace_regex: regex::Regex,
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
                _whitespace_regex: regex::Regex::new(r"\s+")?,
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
            if let Ok(Some(bibliography_start)) = self._bibliography_start_regex.find(chunk) {
                self._bibliography_start_found = true;
                // only process from matched bibliography in the current chunk
                to_process= &chunk[bibliography_start.end()..]
            }
        }

        if self._bibliography_start_found {
            let bibliography_entries = self._bibliography_entry_regex.captures_iter(to_process);
            for bibliography_entry in bibliography_entries {
                if let Ok(cap) = bibliography_entry {
                    let key = cap.get(1).map_or("", |mat| mat.as_str());
                    let value = cap.get(2).map_or("", |mat| mat.as_str()).trim().replace("\n", " ");

                    let formatted_value = self._whitespace_regex.replace_all(&value, " ");

                    if !self._found.contains_key(key) {
                        self._found.insert(String::from(key), String::from((&formatted_value).as_ref()));
                    }
                }
            }
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<json::JsonValue, utils::Error> {
        Ok(JsonValue::from(self._found.drain().collect::<HashMap<String, String>>()))
    }

    fn clear(&mut self){
        self._buffer.clear();
        self._found.clear();
        self._bibliography_start_found = false;
    }
}

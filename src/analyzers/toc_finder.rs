// ^\s+([a-zA-Z0-9\.]*)\s*([\w \“\(\-\)\:\”\.]*?(?=\.{2}))\.*([0-9]*)
use json::{JsonValue};
use fancy_regex::Regex;

use std::collections::HashMap;

use crate::utils;

use super::traits::Analyzer;

pub(crate) struct ToCFinder {

    _toc_entry_regex: Regex,
    _toc_start_regex: Regex,
    _toc_end_regex: Regex,
    _toc_start_found: bool,
    _toc_end_found: bool,
    _found: Vec<(String, String, u32)>,
    _buffer: String // maybe unused

}

impl ToCFinder {

    pub(crate) fn new() -> Result<Self, utils::Error> {

        let vec = Vec::new();
        Ok(
            Self {
                _toc_entry_regex: Regex::new(r"\s*([a-zA-Z0-9.]*)\s*([\w “(\-):”.]*?(?=\.{2}))\.*([0-9]*)")?,
                _toc_start_regex: Regex::new(r"(?i).*(table of contents)\n")?,
                _toc_end_regex: Regex::new(r"\n\n")?,
                _toc_start_found: false,
                _toc_end_found: false,
                _found: vec,
                _buffer: String::new(),
            }
        )
    }
}

impl Analyzer for ToCFinder {

    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {
        let mut to_process = chunk;
        if self._toc_end_found {
            to_process = "";
        }
        if !self._toc_start_found {
            let toc_start = self._toc_start_regex.find(chunk);
            if toc_start.is_ok() {
                let match_option = toc_start.unwrap();
                if match_option.is_some() {
                    self._toc_start_found = true;
                    // only process from matched bibliography in the current chunk
                    let m = match_option.unwrap();
                    to_process= &chunk[m.end()..];
                    // println!("{}", to_process);
                }
            }
        }
        if self._toc_start_found && !self._toc_end_found {
            let toc_end = self._toc_end_regex.find(to_process);
            if toc_end.is_ok() {
                let match_option = toc_end.unwrap();
                if match_option.is_some() {
                    self._toc_end_found = true;
                    // only process from matched bibliography in the current chunk
                    let m = match_option.unwrap();
                    to_process= &to_process[..m.start()];
                    // println!("{}", to_process);
                }
            }
        }

        if self._toc_start_found  {
            println!("{}", String::from(to_process));

            let toc_entries = self._toc_entry_regex.captures_iter(to_process);
            for toc_entry in toc_entries {
                let unwrapped = toc_entry?; // this needs to be handled


                let index = unwrapped.get(1).unwrap().as_str().trim();
                let name = unwrapped.get(2).unwrap().as_str().trim().replace("\n", " ");
                let page = unwrapped.get(3).unwrap().as_str().trim().replace("\n", " ");



                let page_num = page.parse::<u32>().unwrap();
                self._found.push((String::from(index), String::from(name), page_num))

            }
        }
        println!("{}", self._found.len());
        Ok(())
    }

    fn finalize(&mut self) -> Result<json::JsonValue, utils::Error> {
        let map  = self._found.clone();
        // Ok(JsonValue::from(map))
        // Ok(json::JsonValue::from(map))
        Ok(JsonValue::from(self._found.drain(0..).collect::<Vec<_>>()))
    }

    fn clear(&mut self){
        self._buffer.clear();
        self._found.clear();
        self._toc_start_found = false;
    }
}

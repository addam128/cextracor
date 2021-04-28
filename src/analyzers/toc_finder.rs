use json::array;
use fancy_regex::Regex;

use crate::utils;

use super::traits::Analyzer;

pub(crate) struct ToCFinder {

    _toc_entry_regex: Regex,
    _toc_entry_regex_alternative: Regex,
    _toc_start_regex: Regex,
    _toc_start_regex_alternative: Regex,
    _toc_end_regex: Regex,
    _toc_start_found: bool,
    _toc_end_found: bool,
    _alternative: bool,
    _found: Vec<json::JsonValue>

}

impl ToCFinder {

    pub(crate) fn new() -> Result<Self, utils::Error> {

        let vec = Vec::new();
        Ok(
            Self {
                _toc_entry_regex: Regex::new(r"\s*([a-zA-Z0-9.]*[a-zA-Z0-9])\s*\.{0,1}([\w “(\-):”.\/’\[\]–]*?(?=\.{2}))\.*\s?([0-9]*)")?,
                _toc_entry_regex_alternative: Regex::new(r"\s*([a-zA-Z0-9.]*[a-zA-Z0-9])\s*\.{0,1}([\w“ (\-):”.\/’\[\]–]*?(?=\ {3,}))\ *([0-9]*)")?,
                _toc_start_regex: Regex::new(r"(?i)\n\s*(table of contents|contents|content)\n")?,
                _toc_start_regex_alternative: Regex::new(r"(?i)\n\s*(table of contents|contents|content):?\n")?,
                _toc_end_regex: Regex::new(r"\n{3}|.*(TÜV, TUEV).*")?,
                _toc_start_found: false,
                _toc_end_found: false,
                _alternative: false,
                _found: vec
            }
        )
    }
}

impl Analyzer for ToCFinder {

    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {

        let mut to_process = chunk;
        if self._toc_end_found {
            return Ok(())
        }
        if !self._toc_start_found {
            if let Ok(toc_start) =  self._toc_start_regex.find(chunk) {
                match toc_start {
                    Some(mat) => {
                        self._toc_start_found = true;
                        to_process= &chunk[mat.end()..];
                    }
                    None => {
                        if let Ok(Some(mat)) = self._toc_start_regex_alternative.find(chunk) {

                            self._toc_start_found = true;
                            self._alternative = true;
                            to_process= &chunk[mat.end()..];

                        }
                        
                    }
                }
            }
        }

        if self._toc_start_found && !self._toc_end_found {
            if let Ok(Some(toc_end)) = self._toc_end_regex.find(to_process) {
                self._toc_end_found = true;
                to_process = &to_process[..toc_end.start()];
            }
        }

        if self._toc_start_found  {
            let toc_entries;
            if !self._alternative {
                toc_entries = self._toc_entry_regex.captures_iter(to_process);
            } else {
                toc_entries = self._toc_entry_regex_alternative.captures_iter(to_process);
            }
            for cap in toc_entries.flatten() {
                    let index = cap.get(1).map_or("", |mat| mat.as_str()).trim();
                    let name = cap.get(2).map_or("", |mat| mat.as_str()).trim().replace("\n", " ");
                    let page = cap.get(3).map_or("", |mat| mat.as_str()).trim().replace("\n", " ");
                    let page_num = page.parse::<u32>().unwrap_or(0);
                    self._found.push(array![index, name, page_num]);
            }
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<json::JsonValue, utils::Error> {
        Ok(
            json::JsonValue::from(self._found.drain(0..).collect::<Vec<_>>())
        )
    }

    fn clear(&mut self){
        self._found.clear();
        self._toc_start_found = false;
        self._alternative = false;
    }
}

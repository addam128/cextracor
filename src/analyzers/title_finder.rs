use regex::Regex;
use json::{ JsonValue, object };

use crate::utils;

use super::traits::Analyzer;

pub(crate) struct TitleFinder {
    _title_regexes: [Regex; 7],
    _title: String

}

impl TitleFinder {
    pub(crate) fn new() -> Result<Self, utils::Error> {
        Ok(
            Self{
                _title: String::new(),
                _title_regexes: [
                    Regex::new(r"(?i)Security (?i)Target[ Lite]*\s*(.*(\s*.*)*)(Common Criteria|Reference)")?,
                    Regex::new(r"EAL\d\+(.+)Document version")?,
                    Regex::new(r"(\S+(\s*.*)*)\s*Security Target Lite")?, 
                    Regex::new(r"(\S+(\s*.*)*)\s*CC Document")?,
                    Regex::new(r"for\s*(.*(\s*.*)*)from")?,
                    Regex::new(r"EAL6\+\s+(.*(\s*.*)+)\s+H13")?,
                    Regex::new(r"Version \d{4}-\d\s+(.*(\s*.*)*)\s+Sponsor")?]
            }
        )
    }
}

impl Analyzer for TitleFinder {

    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {
        if !self._title.is_empty() {
            return Ok(());
        }
        let mut text = "".to_owned();
        for line in chunk.lines(){
            text.push_str(line);
            for title_regex in &self._title_regexes{
                match title_regex.captures(&text) {
                    None => {continue}
                    Some(_) => {
                        let caps = title_regex.captures(&text).unwrap();
                        let mut i = 1;
                        while caps.get(i) == None {
                            i = i + 1;
                        }
                        self._title = caps.get(i).map_or("", |m| m.as_str()).replace("\n", "");
                        return Ok(())
                    }
                }
            }
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<JsonValue, utils::Error> {
        let re = Regex::new(r"\s+").unwrap();
        let result_title = re.replace_all(&self._title, " ");
        Ok(
            JsonValue::from(result_title.trim())
        )
    }

    fn clear(&mut self) -> () {}
}
use json::object;
use json::array;
use regex::{Regex, RegexSet};

use crate::utils;
use crate::models::revision::Revision;
use super::traits::Analyzer;

const MAX_INTERNAL_BUFFER: usize = 3072; // under this one document fails because the revisions is too long

pub(crate) struct RevisionsFinder {

    _activated: bool,
    _done: bool,
    _revisions: Vec<Revision>,
    _revision_lines: Vec<String>,
    _dummy_string: String,
    _revision_regex: Vec<Regex>,
    _activator_regex: Regex,
    _buffer: String

}

impl RevisionsFinder {

    pub(crate) fn new() -> Result<Self, utils::Error>  {
       
        Ok(
            Self {
                _activated: false,
                _done: false,
                _revisions: Vec::new(),
                _revision_lines: Vec::new(),
                _dummy_string: String::new(),
                _revision_regex: vec![
                    Regex::new(
                    r"^(?:\d{2,4}[-/\.](\d{2}|\w{2,10})[-/.]\d{2,4})\s+(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+\s+[^\n]+"
                    )?,
                    Regex::new(r"^(?:(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))\s+(?:\d{2,4}[-/\.](\d{2}|\w{2,10})[-/.]\d{2,4}):?\s+[^\n]+"
                    )?,
                    Regex::new(
                        r"^(?:(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))\s+[^\n]+"
                    )?
                ], // regex set could help
                _activator_regex: Regex::new(
                    /* mandatory newline at the end eliminates these titles if they are in contents table */
                    r"(?i)(?:Revision\s+?History|(?-i)Version\s+?Control|(?i)Document\s+?Evolution|Rev\s+?Date\s+?(?:Authors\s+?)?Description):?\s*\n")?,
                _buffer: String::new()
            }
        )
    }

    fn process_buffered(&mut self) -> Result<(), utils::Error> {

        let mut empty = true;

        for line in self._buffer.lines().into_iter().map(|l| l.trim()) {
            if line == "" {
                if empty { continue }
                else { break; }
            }

            if self._revision_regex.iter()
                                   .map(|reg| reg.is_match(line))
                                   .fold(false, |acc, elem| acc || elem) {

                empty = false;
                self._revision_lines.push(String::from(line));
                continue;

            }

            self._revision_lines.last_mut() // if ther is a revision add cont. line, if not ad to dummy
                    .unwrap_or(&mut self._dummy_string).push_str(&format!(" {}", line));

            self._dummy_string.clear(); // remove dummy to keep it small
        }

        Ok(())
    }

    fn find_lines(
        &mut self,
        chunk: &str,
        start_pos: usize)
        -> Result<(), utils::Error>{
        
        if self._buffer.len() >= MAX_INTERNAL_BUFFER {
            self._done = true;
            return Ok(());
        }

        let remainder = MAX_INTERNAL_BUFFER - self._buffer.len();

        let mut end_pos = start_pos + remainder;
        if chunk[start_pos..].len() < remainder {
            end_pos = chunk.len();
        }

        self._buffer.push_str(&chunk[start_pos..end_pos]);
        
        Ok(())
    }


    fn find_block(&mut self, chunk: &str) -> Result<(), utils::Error> {
        
        if let Some(mat) = self._activator_regex.find(chunk) {
            self._activated = true;
            self.find_lines(chunk, mat.end())

        } else {
            return Ok(());
        }
    }
}

impl Analyzer for RevisionsFinder {
    
    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {
        
        if self._done {
            return Ok(())
        }

        match self._activated {

            true => {
                self.find_lines(chunk, 0)?;
            },
            false => {
                self.find_block(chunk)?;
            }
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<json::JsonValue, utils::Error> {
        
        self.process_buffered()?;
        for line in self._revision_lines.iter() {
            println!("{}", line);
        }
        Ok(array!{})
    }

    fn clear(&mut self) {

        self._buffer.clear();
        self._revisions.clear();
        self._revision_lines.clear();
        self._dummy_string.clear();
        self._done = false;
        self._activated = false;
    }
}
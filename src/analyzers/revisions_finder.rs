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
    _revision_regex: RegexSet,
    _activator_regex: Regex,
    _date_regex: Regex,
    _id_regex: Regex,
    _buffer: String,
    _whitespace_regex: Regex,
    _date_deconstruct_regex: Regex

}

impl RevisionsFinder {

    pub(crate) fn new() -> Result<Self, utils::Error>  {
       
        Ok(
            Self {
                _activated: false,
                _done: false,
                _revisions: Vec::new(),
                _revision_lines: Vec::new(),
                _revision_regex: RegexSet::new( // all possible combinations for revision line(s)
                    &[
                    r"^(?:\d{2,4}[-/\.](\d{2}|\w{2,10})[-/.]\d{2,4})\s+(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+\s+[^\n]+",
                    r"^(?:(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))\s+(?:\d{2,4}[-/\.](\d{2}|\w{2,10})[-/.]\d{2,4}):?\s+[^\n]+",
                    r"^(?:(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))\s+[^\n]+"
                    ]
                )?,
                _activator_regex: Regex::new(
                    /* mandatory newline at the end eliminates these titles if they are in contents table, but its not bulletproof, so rather check some data after each found */
                    r"(?i)(?:Revision\s+?History|(?-i)Version\s+?Control|(?i)Document\s+?Evolution|Rev\s+?Date\s+?(?:Authors\s+?)?Description):?\s*\n")?,
                _buffer: String::new(),
                _date_regex: Regex::new(r"^(?:\d{2,4}[-/\.](\d{2}|\w{2,10})[-/.]\d{2,4})")?, // end not anchored, possible ":"
                _id_regex: Regex::new(r"^(?:(?:Version|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))")?,
                _whitespace_regex: Regex::new(r"\s{2,}")?,
                _date_deconstruct_regex: Regex::new(r"^((?P<year>\d{4})[\.-/](?P<month>\d{1,2}|\w{2,12})[\.-/](?P<day>\d{1,2})|(?P<day>\d{1,2})[\.-/](?P<month>\d{1,2}|\w{2,12})[\.-/](?P<year>\d{4}))$")?

            }
        )
    }

    fn process_buffered(&mut self) -> Result<(), utils::Error> {

        let mut empty = true;
        let mut black_hole = String::new();

        for line in self._buffer.lines().into_iter().map(|l| l.trim()) {
            if line == "" { // this is working but kinda meh, maybe find something better to stop looking for revisions
                if empty { continue }
                else { break; }
            }

            if self._revision_regex.is_match(line) {

                empty = false;
                self._revision_lines.push(String::from(line));
                continue;

            }

            self._revision_lines.last_mut() // if ther is a revision add cont. line, if not ad to dummy
                    .unwrap_or(&mut black_hole).push_str(&format!(" {}", line));

            black_hole.clear(); // remove dummy to keep it small
        }

        self._buffer.clear();

        Ok(())
    }

    fn find_lines(
        &mut self,
        chunk: &str,
        start_pos: usize)
        -> Result<(), utils::Error>{
        
        if self._buffer.len() >= MAX_INTERNAL_BUFFER {
            self._activated = false;
            return self.process_buffered();
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

    fn construct_revisions(&mut self) -> Result<(), utils::Error> {

        let mut date: &str = "";
        let mut id: &str = "";
        let mut desc: &str = ""; 

        for rev_line in self._revision_lines.iter() {

            let parts = self._whitespace_regex.split(rev_line.as_str());

            for part in parts {

                if self._date_regex.is_match(part) {
                    date = part;
                }
                else if self._id_regex.is_match(part) {
                    id = part;
                }
                else {
                    desc = part; // because description seems to be always after author, the for cycle naturally will put that in desc at the end of the for loop
                }
            }



            self._revisions.push(Revision::new(id, desc, date));
        }

        Ok(())
    }
}

impl Analyzer for RevisionsFinder {
    
    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {

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
        self.construct_revisions()?;
        Ok(
            json::JsonValue::from(self._revisions.drain(0..).collect::<Vec<_>>())
        )
    }

    fn clear(&mut self) {

        self._buffer.clear();
        self._revisions.clear();
        self._revision_lines.clear();
        self._done = false;
        self._activated = false;
    }
}
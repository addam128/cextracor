use std::str::from_utf8;

use regex::{Regex, RegexSet};

use crate::utils;
use crate::models::revision::Revision;
use crate::models::date::DateFormatter;
use super::traits::Analyzer;


const MAX_INTERNAL_BUFFER: usize = 3072; // under this one document fails because the revisions is too long

pub(crate) struct RevisionsFinder {

    _activated: bool,
    _done: bool,
    _revisions: Vec<Revision>,
    _revision_lines: Vec<String>,
    _revision_regex: RegexSet,
    _activator_regex: Regex,
    _date_match_regex: Regex,
    _date_find_regex: Regex,
    _id_regex: Regex,
    _buffer: String,
    _whitespace_regex: Regex,
    _date_formatter: DateFormatter,
    _id_num_regex: Regex

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
                    r"^(?:\d{2,4}[-/\.\s](\d{2}|\w{2,10})[-/\.\s]\d{2,4})\s+(?:Version\s?|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+\s+[^\n]+",
                    r"^(?:(?:Version\s?|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|(?i)Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))\s+(?:\d{2,4}[-/\s\.](\d{2}|\w{2,10})[-/\s\.]\d{2,4}):?\s+[^\n]+",
                    r"^(?:(?:Version\s?|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|(?i)Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))\s+[^\n]+"
                    ]
                )?,
                _activator_regex: Regex::new(
                    /* mandatory newline at the end eliminates these titles if they are in contents table, but its not bulletproof, so rather check some data after each found */
                    r"(?i)(?:Revision\s+?History|(?-i)Version\s+?Control|(?i)Document\s+?Evolution|Rev\s+?Date\s+?(?:Authors\s+?)?Description):?\s*\n")?,
                _buffer: String::new(),
                _date_match_regex: Regex::new(r"^(?:\d{2,4}[-/\.\s](\d{2}|\w{2,10})[-/\.\s]\d{2,4}):?$")?,
                _date_find_regex: Regex::new(r"\(?(?:\d{2,4}[-/\.\s](\d{2}|\w{2,10})[-/\.\s]\d{2,4})\)?:?")?,
                _id_regex: Regex::new(r"^(?:(?:Version\s?|v)?(?:\d{1,2}\.)(?:\d{1,2}\.?)?+|(?i)Rev\.?\s*([A-Z]|(?:\d{1,2}\.?)+))$")?,
                _whitespace_regex: Regex::new(r"\s{2,}")?,
                _date_formatter: DateFormatter::new()?,
                _id_num_regex: Regex::new(r"(\d{1,2}\.(\d\.?)*)")?

            }
        )
    }

    fn process_buffered(&mut self) -> Result<(), utils::Error> {

        let mut empty = true;
        let mut black_hole = String::new();
        let mut empty_counter = 0;

        for line in self._buffer.lines().into_iter().map(|l| l.trim()) {
            if line.is_empty() { // this is working but kinda meh, maybe find something better to stop looking for revisions
                if empty { continue }
                if empty_counter < 2 {
                    empty_counter += 1;
                    continue;
                }
                else { break; }
            }

            if self._revision_regex.is_match(line) {

                empty = false;
                self._revision_lines.push(String::from(line));
                continue;

            }

            self._revision_lines.last_mut() // if ther is a revision add cont. line, if not add to dummy
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

        let remainder = MAX_INTERNAL_BUFFER - self._buffer.len(); // type overflow checked above

        let mut end_pos = start_pos + remainder;

        if chunk[start_pos..].len() <= remainder {
            end_pos = chunk.len();
        }

        let mut counter = 0;

        loop {
            if counter > 3 {
                return Err(utils::Error::BadRead);
            }

            match from_utf8(&chunk.as_bytes()[start_pos..end_pos]) {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    end_pos += 1;
                }
            }
            counter += 1;
        }
        // the above loop block is not needed if u use:
        //chunk[start_pos..].chars().take(end_pos-start_pos).for_each(|ch| self._buffer.push(ch)); this works as well but thewn the buffer limit is char and not byte based

        self._buffer.push_str(&chunk[start_pos..end_pos]);
        
        Ok(())
    }


    fn find_block(&mut self, chunk: &str) -> Result<(), utils::Error> {
        
        if let Some(mat) = self._activator_regex.find(chunk) {
            self._activated = true;
            self.find_lines(chunk, mat.end())
        } 
        else {
            Ok(())
        }
    }

    fn construct_revisions(&mut self) -> Result<(), utils::Error> {

        for rev_line in self._revision_lines.iter() {

            let mut date = String::new();
            let mut id = String::new();
            let mut desc = String::new(); 

            let parts = self._whitespace_regex.split(rev_line.as_str()).take(4);

            for part in parts {

                if self._date_match_regex.is_match(part) {
                    date.push_str(part);
                }
                else if self._id_regex.is_match(part) {
                    id.push_str(part);

                    if let Some(mat) = self._id_num_regex.find(part) {
                        id.clear();
                        id.push_str(&part[mat.start()..mat.end()]);
                    }
                }
                else {
                    desc.clear();
                    desc.push_str(part); // because description seems to be always after author, the for cycle naturally will put that in desc at the end of the for loop
                                // however if trash is also collected(shouldnt be the case unless attack), this will yield something else than the description
                                // but we dont care too much about results on malicious file
                }

            }

            if date.is_empty()  && !desc.is_empty() {
                    
                if let Some(mat) = self._date_find_regex.find(desc.as_str()) {

                    date.push_str(&desc[mat.start()..mat.end()]);
                    desc = format!("{}{}", &desc[..mat.start()], &desc[mat.end()..].trim());
                }

        }

        if id.is_empty() { // if no version then simply dont output
            continue;
        }

            self._revisions.push(
                Revision::new(
                    id,
                    desc,
                    self._date_formatter.standardize(date.as_str())
                )
            );
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
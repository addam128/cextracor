use json::{JsonValue, object};
use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::utils;

use super::traits::Analyzer;

pub(crate) struct VersionFinder {
    
    _rsa_regex: Regex,
    _eal_regex: Regex,
    _ecc_regex: Regex,
    _des_regex: Regex,
    _sha_regex: Regex,
    _java_card_regex: Regex,
    _global_platform_regex: Regex,
    _found: HashMap< String, HashSet<String> >,

}

impl VersionFinder {

    pub(crate) fn new() -> Result<Self, utils::Error> {

        let mut key_map = HashMap::new();
        key_map.insert(String::from("rsa"), HashSet::new());
        key_map.insert(String::from("eal"), HashSet::new());
        key_map.insert(String::from("ecc"), HashSet::new());
        key_map.insert(String::from("des"), HashSet::new());
        key_map.insert(String::from("sha"), HashSet::new());
        key_map.insert(String::from("java_card"), HashSet::new());
        key_map.insert(String::from("global_platform"), HashSet::new());
        Ok(
            Self {
                _rsa_regex: Regex::new(r"R(?i)sa(?-i)((\s?-?[A-Z0-9]{2,4})\s?(-?\s?\d{1,4}?(/\d{0,4})?)|(-?\s?\d{1,4}?(/\d{0,4})?)|(\s?-?[A-Z0-9]{2,4})){1}")?,
                _eal_regex: Regex::new(r"E(?i)al\s?-?\d{1}\s?\+?")?,
                _ecc_regex: Regex::new(r"(?i)ecc(\s?-?\d{0,8})")?,
                _des_regex: Regex::new(r"(?i)(Triple|Double|3-key\s?T?|3|(?-i)T(?i)|2-key\s?T?|Single|SW){1}-?\s?Des")?,
                _sha_regex: Regex::new(r"S(?i)ha\d?\s?(-?\s?\d?/?\d{1,8})")?,
                _java_card_regex: Regex::new(r"(?i)java\s?card\s?-?(\d\.?){1,8}")?,
                _global_platform_regex: Regex::new(r"(?i)global\s?-?platform\s?-?(\d\.?){1,8}")?,
                _found: key_map,
            }
    )
        
    }
}

impl Analyzer for VersionFinder {

    fn process(&mut self, chunk: &str) -> Result<(), utils::Error> {
        
        let eal_iter = self._eal_regex.find_iter(chunk);
        for mat in eal_iter {
            self._found.get_mut("eal").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim().to_uppercase())
                                .replace("\n", " "));
        }

        let rsa_iter = self._rsa_regex.find_iter(chunk);
        for mat in rsa_iter {
            self._found.get_mut("rsa").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim().to_uppercase())
                                .replace("\n", " "));               
        }

        let sha_iter = self._sha_regex.find_iter(chunk);
        for mat in sha_iter {
            self._found.get_mut("sha").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim().to_uppercase())
                                .replace("\n", " "));               
        }

        let des_iter = self._des_regex.find_iter(chunk);
        for mat in des_iter {
            self._found.get_mut("des").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim())
                                .replace("\n", " "));
        }        
                
        let ecc_iter = self._ecc_regex.find_iter(chunk);
        for mat in ecc_iter {
            self._found.get_mut("ecc").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim().to_uppercase())
                                .replace("\n", " "));    
        }
    
        let jc_iter = self._java_card_regex.find_iter(chunk);
        for mat in jc_iter {
            self._found.get_mut("java_card").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim())
                                .replace("\n", " "));    
        }

        let gp_iter = self._global_platform_regex.find_iter(chunk);
        for mat in gp_iter {
            self._found.get_mut("global_platform").unwrap()
                .insert(String::from(chunk[mat.start()..mat.end()].trim())
                                .replace("\n", " "));    
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<JsonValue, utils::Error> {

        Ok(object! {
            eal: Vec::from_iter(self._found.get_mut("eal").unwrap().drain()),
            rsa: Vec::from_iter(self._found.get_mut("rsa").unwrap().drain()),
            des: Vec::from_iter(self._found.get_mut("des").unwrap().drain()),
            sha: Vec::from_iter(self._found.get_mut("sha").unwrap().drain()),
            ecc: Vec::from_iter(self._found.get_mut("ecc").unwrap().drain()),
            java_card: Vec::from_iter(self._found.get_mut("java_card").unwrap().drain()),
            global_platform: Vec::from_iter(self._found.get_mut("global_platform").unwrap().drain())
            }
        )
    }

    fn clear(&mut self) -> () {
        for set in self._found.values_mut() {
            set.clear();
        }
    }
}
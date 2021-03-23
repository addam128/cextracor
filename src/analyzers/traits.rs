use crate::utils;

use json::JsonValue;
pub(crate) trait Analyzer {
    
    fn process(&mut self, chunk: &str) -> Result<(), utils::Error>;
    fn finalize<J>(&self) -> Result<Vec<J>, utils::Error> 
        where J: Into<JsonValue>;
}
use crate::utils;

use json::JsonValue;
pub(crate) trait Analyzer {
    
    fn process(&mut self, chunk: &str) -> Result<(), utils::Error>;
    
    fn finalize(&mut self) -> Result<JsonValue, utils::Error>;

    fn clear(&mut self) -> ();
}

use json::{self, JsonValue};

pub(crate) struct Revision {

    _version: String,
    _description: String,
    _date: String
}

impl Revision {
   
    pub(crate) fn new(
        version: &str,
        description: &str,
        date: &str)
        -> Self {

            // TODO: if needed transform date into the right format
            Self {
                _version: String::from(version),
                _description: String::from(description),
                _date: String::from(date)
            }
        }
}

impl Into<JsonValue> for Revision {

    fn into(self) -> JsonValue {

        json::object! {
            version: self._version,
            description: self._description,
            date: self._date
        }
    }
}

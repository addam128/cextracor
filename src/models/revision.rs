use json::{self, JsonValue};

pub(crate) struct Revision {

    _version: String,
    _description: String,
    _date: String
}

impl Revision {
   
    pub(crate) fn new(
        version: String,
        description: String,
        date: String)
        -> Self {

            // TODO: if needed transform date into the right format
            Self {
                _version: version,
                _description: description,
                _date: date
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

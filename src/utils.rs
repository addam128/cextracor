
#[derive(Debug)]
pub(crate) enum Error {

    Io(std::io::Error),
    Utf8Conversion(std::str::Utf8Error),
    IsADirectory,
    Regex(regex::Error),
    FancyRegex(fancy_regex::Error),
    BadRead,
    UserChoice
}

impl From<std::io::Error> for Error {

    fn from(err: std::io::Error) -> Self {Error::Io(err)}
}

impl From<std::str::Utf8Error> for Error {

    fn from(err: std::str::Utf8Error) -> Self {Error::Utf8Conversion(err)}
}

impl From<regex::Error> for Error {

    fn from(err: regex::Error) -> Self {Error::Regex(err)}
}
impl From<fancy_regex::Error> for Error {

    fn from(err: fancy_regex::Error) -> Self {Error::FancyRegex(err)}
}

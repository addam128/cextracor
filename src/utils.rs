
#[derive(Debug)]
pub(crate) enum Error {

    IOError(std::io::Error),
    Utf8ConversionError(std::str::Utf8Error)
}

impl From<std::io::Error> for Error {

    fn from(err: std::io::Error) -> Self {Error::IOError(err)}
}

impl From<std::str::Utf8Error> for Error {

    fn from(err: std::str::Utf8Error) -> Self {Error::Utf8ConversionError(err)}
}
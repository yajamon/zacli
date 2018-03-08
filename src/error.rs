#[derive(Debug)]
pub enum Error {
    SimpleError(String),
    IoError(::std::io::Error),
    TomlDeserializeError(::toml::de::Error),
    TomlSerializeError(::toml::ser::Error),
}
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::SimpleError(error)
    }
}
impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<::toml::de::Error> for Error {
    fn from(error: ::toml::de::Error) -> Self {
        Error::TomlDeserializeError(error)
    }
}

impl From<::toml::ser::Error> for Error {
    fn from(error: ::toml::ser::Error) -> Self {
        Error::TomlSerializeError(error)
    }
}

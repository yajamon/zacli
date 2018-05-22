#[derive(Debug)]
pub enum Error {
    SimpleError(String),
    IoError(::std::io::Error),
    TomlDeserializeError(::toml::de::Error),
    TomlSerializeError(::toml::ser::Error),
}
pub type Result<T> = ::std::result::Result<T, Error>;

macro_rules! impl_from_for_errors {
    ($($id:ident : $type:ty => $translate:block),*) => {
        $(
            impl From<$type> for Error {
                fn from($id: $type) -> Self $translate
            }
        )*
    }
}

impl_from_for_errors!(
    error: String => { Error::SimpleError(error) },
    error: ::std::io::Error => { Error::IoError(error) },
    error: ::toml::de::Error => { Error::TomlDeserializeError(error) },
    error: ::toml::ser::Error => { Error::TomlSerializeError(error) }
);

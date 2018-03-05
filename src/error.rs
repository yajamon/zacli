#[derive(Debug)]
pub enum Error {
    SimpleError(String),
    IoError(::std::io::Error),
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

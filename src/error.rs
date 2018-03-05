#[derive(Debug)]
pub enum Error {
    SimpleError(String),
}
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::SimpleError(error)
    }
}

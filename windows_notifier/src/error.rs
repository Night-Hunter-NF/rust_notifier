#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    XmlErr(XmlErr),
    Windows(windows::core::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XmlErr {
    InvatedArg(String),
}

impl From<XmlErr> for Error {
    fn from(err: XmlErr) -> Self {
        Error::XmlErr(err)
    }
}

impl From<windows::core::Error> for Error {
    fn from(err: windows::core::Error) -> Self {
        Error::Windows(err)
    }
}


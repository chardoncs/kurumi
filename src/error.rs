use std::fmt::Debug;

pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[error] ({:#?}) {}", self.kind(), self.message()).as_str())
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Window,
    File,
    UrlParsing,
}

#[macro_export]
macro_rules! mismatching_error {
    ($target:literal) => {
        concat!("GTK object mismatched: expecting a `", $target, "`")
    };
}


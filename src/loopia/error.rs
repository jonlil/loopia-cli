use serde::Serialize as DeriveSerialize;

#[derive(Debug, PartialEq)]
pub struct Error(LoopiaErrorKind);

impl Error {
    pub fn new(error: LoopiaErrorKind) -> Self {
        Error(error)
    }

    pub fn inner(&self) -> &LoopiaErrorKind {
        &self.0
    }
}

#[derive(Debug, PartialEq, DeriveSerialize)]
pub enum LoopiaErrorKind {
    AuthenticationError,
    UnknownError,
    RuntimeError,
}

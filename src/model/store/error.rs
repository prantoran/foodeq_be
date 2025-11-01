

use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

// The errors will not be sent to the client directly, hence using Serialize only.
#[derive(Debug, Serialize, Clone)]
pub enum Error {
    FailToCreatePool(String),
}

// region:   --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion:   --- Error Boilerplate
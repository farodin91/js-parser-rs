pub mod error;

use error::error::Error;

pub type JsResult<T> = Result<T, Error>;
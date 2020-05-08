use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Error as IOError;

use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum TreasureError {
    IO(IOError),
    JSON(JsonError)
}

impl Display for TreasureError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::IO(io) => write!(f, "An IO error occurred: {}", io),
            Self::JSON(json) => write!(f, "A JSON error occurred: {}", json)
        }
    }
}

impl std::error::Error for TreasureError {}

impl From<IOError> for TreasureError {
    fn from(io: IOError) -> Self { TreasureError::IO(io) }
}

impl From<JsonError> for TreasureError {
    fn from(json: JsonError) -> Self { TreasureError::JSON(json) }
}

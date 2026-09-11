use std::{error::Error, fmt};

#[derive(Debug)]
pub enum MemoryError {
    Validation(String),
    CapacityExceeded,
    DuplicatePrediction(String),
    UnknownPrediction(String),
    OutcomeDoesNotMatchPrediction(String),
    Io(std::io::Error),
    Serialization(serde_json::Error),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) => {
                write!(formatter, "invalid evidence memory input: {message}")
            }
            Self::CapacityExceeded => write!(formatter, "evidence memory record limit exceeded"),
            Self::DuplicatePrediction(id) => write!(formatter, "prediction already exists: {id}"),
            Self::UnknownPrediction(id) => write!(formatter, "unknown prediction: {id}"),
            Self::OutcomeDoesNotMatchPrediction(id) => {
                write!(formatter, "outcome does not match prediction: {id}")
            }
            Self::Io(error) => write!(formatter, "evidence memory I/O error: {error}"),
            Self::Serialization(error) => {
                write!(formatter, "evidence memory serialization error: {error}")
            }
        }
    }
}

impl Error for MemoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for MemoryError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for MemoryError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SynthParseError {
    InvalidWaveform,
    InvalidFilterMode,
}

impl Display for SynthParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWaveform => f.write_str("Invalid waveform."),
            Self::InvalidFilterMode => f.write_str("Invalid filter mode."),
        }
    }
}

impl Error for SynthParseError {}

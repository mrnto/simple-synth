use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::SynthParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterMode {
    Lowpass,
    Highpass,
    Bandpass,
}

impl Display for FilterMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Lowpass => "lowpass",
            Self::Highpass => "highpass",
            Self::Bandpass => "bandpass",
        })
    }
}

impl FromStr for FilterMode {
    type Err = SynthParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "lowpass" => Ok(Self::Lowpass),
            "highpass" => Ok(Self::Highpass),
            "bandpass" => Ok(Self::Bandpass),
            _ => Err(SynthParseError::InvalidFilterMode),
        }
    }
}

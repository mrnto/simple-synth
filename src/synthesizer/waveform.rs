use std::fmt::{Display, Formatter};
use std::str::FromStr;
use nih_plug::prelude::Enum;
use crate::error::SynthParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Square"]
    Square,
    #[name = "Triangle"]
    Triangle,
    #[name = "Sawtooth"]
    Sawtooth,
    #[name = "Noise"]
    Noise,
}

impl Display for Waveform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Sine => "sine",
            Self::Square => "square",
            Self::Triangle => "triangle",
            Self::Sawtooth => "sawtooth",
            Self::Noise => "noise",
        })
    }
}

impl FromStr for Waveform {
    type Err = SynthParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "sine" => Ok(Self::Sine),
            "square" => Ok(Self::Square),
            "triangle" => Ok(Self::Triangle),
            "sawtooth" => Ok(Self::Sawtooth),
            "noise" => Ok(Self::Noise),
            _ => Err(SynthParseError::InvalidWaveform),
        }
    }
}

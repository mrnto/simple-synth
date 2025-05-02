use std::str::FromStr;
use crate::error::ParseWaveformError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

impl FromStr for Waveform {
    type Err = ParseWaveformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sine" => Ok(Self::Sine),
            "square" => Ok(Self::Square),
            "triangle" => Ok(Self::Triangle),
            "sawtooth" => Ok(Self::Sawtooth),
            "noise" => Ok(Self::Noise),
            _ => Err(ParseWaveformError),
        }
    }
}

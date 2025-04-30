use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

pub struct ParseWaveformError;

impl FromStr for Waveform {
    type Err = ParseWaveformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sine" => Ok(Self::Sine),
            "Square" => Ok(Self::Square),
            "Triangle" => Ok(Self::Triangle),
            "Sawtooth" => Ok(Self::Sawtooth),
            "Noise" => Ok(Self::Noise),
            _ => Err(ParseWaveformError),
        }
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter};
use cpal::{
    BuildStreamError, DefaultStreamConfigError, DeviceNameError, PauseStreamError, PlayStreamError,
};

#[derive(Debug)]
pub enum AudioEngineError {
    BuildStreamError(BuildStreamError),
    DefaultStreamConfigError(DefaultStreamConfigError),
    DeviceNameError(DeviceNameError),
    NoDefaultDevice,
    PlayStreamError(PlayStreamError),
    PauseStreamError(PauseStreamError),
    SenderUnavailable,
}

impl Display for AudioEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildStreamError(err) => err.fmt(f),
            Self::DefaultStreamConfigError(err) => err.fmt(f),
            Self::DeviceNameError(err) => err.fmt(f),
            Self::NoDefaultDevice => f.write_str("No default output device available."),
            Self::PlayStreamError(err) => err.fmt(f),
            Self::PauseStreamError(err) => err.fmt(f),
            Self::SenderUnavailable => f.write_str("Message sender is not available."),
        }
    }
}

impl Error for AudioEngineError {}

macro_rules! impl_from {
    ($target:ident: $($src:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$src> for $target {
                fn from(err: $src) -> Self {
                    $target::$variant(err)
                }
            }
        )*
    };
}

impl_from!(AudioEngineError:
    BuildStreamError => BuildStreamError,
    DefaultStreamConfigError => DefaultStreamConfigError,
    DeviceNameError => DeviceNameError,
    PlayStreamError => PlayStreamError,
    PauseStreamError => PauseStreamError,
);

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

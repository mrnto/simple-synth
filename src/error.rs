use std::error::Error;
use std::fmt::{Display, Formatter};
use cpal::{
    BuildStreamError, DefaultStreamConfigError, DeviceNameError,
    PlayStreamError, PauseStreamError
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
    ($src:ty, $variant:ident) => {
        impl From<$src> for AudioEngineError {
            fn from(err: $src) -> Self {
                AudioEngineError::$variant(err)
            }
        }
    };
}

impl_from!(BuildStreamError, BuildStreamError);
impl_from!(DefaultStreamConfigError, DefaultStreamConfigError);
impl_from!(DeviceNameError, DeviceNameError);
impl_from!(PlayStreamError, PlayStreamError);
impl_from!(PauseStreamError, PauseStreamError);

#[derive(Copy, Clone, Debug)]
pub struct ParseWaveformError;

impl Display for ParseWaveformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid waveform.")
    }
}

impl Error for ParseWaveformError {}

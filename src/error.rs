use cpal::{DefaultStreamConfigError, DeviceNameError};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DeviceSetupError {
    DefaultStreamConfigError(DefaultStreamConfigError),
    DeviceNameError(DeviceNameError),
    NoDefaultDevice,
}

impl Display for DeviceSetupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefaultStreamConfigError(err) => err.fmt(f),
            Self::DeviceNameError(err) => err.fmt(f),
            Self::NoDefaultDevice => f.write_str("No default output device available."),
        }
    }
}

impl Error for DeviceSetupError {}

impl From<DefaultStreamConfigError> for DeviceSetupError {
    fn from(err: DefaultStreamConfigError) -> Self {
        DeviceSetupError::DefaultStreamConfigError(err)
    }
}

impl From<DeviceNameError> for DeviceSetupError {
    fn from(err: DeviceNameError) -> Self {
        DeviceSetupError::DeviceNameError(err)
    }
}

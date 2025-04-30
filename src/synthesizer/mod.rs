mod envelope;
mod oscillator;
mod synthesizer;
mod voice_manager;
mod voice;
pub mod waveform;

pub use synthesizer::Synthesizer;
pub use waveform::{Waveform, ParseWaveformError};

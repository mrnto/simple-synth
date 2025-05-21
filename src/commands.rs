use crate::synthesizer::{EnvelopeStage, FilterMode, Waveform};

#[derive(Debug, Clone, Copy)]
pub enum SynthParam {
    EnvelopeStage(EnvelopeStage, f32),
    Waveform(Waveform),
    FilterMode(FilterMode),
    Cutoff(f32),
    Resonance(f32),
}

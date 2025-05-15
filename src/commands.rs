use crate::synthesizer::{EnvelopeStage, FilterMode, Waveform};

#[derive(Debug)]
pub enum SynthCommand {
    NoteOn(u8),
    NoteOff(u8),
    SetParam(SynthParam),
}

#[derive(Debug, Clone, Copy)]
pub enum SynthParam {
    EnvelopeStage(EnvelopeStage, f32),
    Waveform(Waveform),
    FilterMode(FilterMode),
    Cutoff(f32),
    Resonance(f32),
}

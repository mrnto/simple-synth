use crate::synthesizer::{EnvelopeStage, Waveform};

#[derive(Debug)]
pub enum SynthMsg {
    NoteOn(u8),
    NoteOff(u8),
    SetStage(EnvelopeStage, f32),
    SetWaveform(Waveform),
}

use crate::synthesizer::{EnvelopeStage, Waveform};

#[derive(Debug)]
pub enum SynthMsg {
    EnvelopeMsg(EnvelopeMsg),
    OscillatorMsg(OscillatorMsg),
    NoteOn(u8),
    NoteOff(u8),
}

#[derive(Debug)]
pub enum EnvelopeMsg {
    SetStage(EnvelopeStage, f32),
}

#[derive(Debug)]
pub enum OscillatorMsg {
    SetWaveform(Waveform),
}

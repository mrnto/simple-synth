use crate::synthesizer::Waveform;

pub enum SynthMsg {
    EnvelopeMsg(EnvelopeMsg),
    OscillatorMsg(OscillatorMsg),
}

pub enum EnvelopeMsg {
    SetAttack(f32),
    SetDecay(f32),
    SetSustain(f32),
    SetRelease(f32),
}

pub enum OscillatorMsg {
    // TODO
    NoteOn(u8),
    NoteOff(u8),
    SetWaveform(Waveform),
    SetOscillator(u32, Waveform, f32),
}

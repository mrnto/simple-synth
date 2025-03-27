use crate::oscillator::Waveform;

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
    SetFrequency(f32),
    SetWaveform(Waveform),
    SetSampleRate(u32),
    SetOscillator(u32, Waveform, f32),
}

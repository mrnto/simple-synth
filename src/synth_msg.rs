use crate::oscillator::Waveform;

pub enum SynthMsg {
    SetFrequency(f32),
    SetSampleRate(u32),
    SetWaveform(Waveform),
    SetOscillator(Waveform, f32, u32),
}

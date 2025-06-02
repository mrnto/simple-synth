mod envelope;
mod filter;
mod oscillator;
mod voice;
mod voice_manager;

pub use envelope::EnvelopeStage;
pub use filter::FilterMode;
pub use oscillator::Waveform;
pub use voice_manager::VoiceManager;

#[derive(Debug, Clone, Copy)]
pub enum SynthParam {
    EnvelopeStage(EnvelopeStage, f32),
    Waveform(Waveform),
    FilterMode(FilterMode),
    Cutoff(f32),
    Resonance(f32),
    SampleRate(f32),
}

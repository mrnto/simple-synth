mod linear;

// TODO: exponential
pub use linear::LinearEnvelope;

/// Represents the different stages of an envelope generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnvelopeStage {
    /// Idle phase, level is 0.0 (no sound, envelope generator is inactive).
    Idle,
    /// Attack phase, level rises from 0.0 to 1.0.
    Attack,
    /// Decay phase, level falls from 1.0 to the sustain level.
    Decay,
    /// Sustain phase, level stays constant at the sustain level.
    Sustain,
    /// Release phase, level falls from sustain level to 0.0.
    Release,
}

/// Trait defining an envelope generator (ADSR).
pub trait Envelope {
    /// The output sample type produced by the envelope generator.
    type Output;

    /// Advances the envelope and returns the current output level (or levels).
    ///
    /// Returns a value of type `Output`.
    fn process(&mut self) -> Self::Output;
    /// Starts the envelope by entering the [`EnvelopeStage::Attack`] stage.
    fn trigger(&mut self);
    /// Starts the [`EnvelopeStage::Release`] stage of the envelope generator.
    fn release(&mut self);
    /// Returns `true` if the envelope is in the [`EnvelopeStage::Idle`] stage.
    fn is_idle(&self) -> bool;
    /// Sets the attack time of the envelope in milliseconds.
    fn set_attack_time(&mut self, attack_time: f32);
    /// Sets the decay time of the envelope in milliseconds.
    fn set_decay_time(&mut self, decay_time: f32);
    /// Sets the sustain level.
    fn set_sustain_level(&mut self, sustain_level: f32);
    /// Sets the release time of the envelope in milliseconds.
    fn set_release_time(&mut self, release_time: f32);
    /// Sets the sample rate in hertz.
    fn set_sample_rate(&mut self, sample_rate: f32);
    /// Resets the internal state of the envelope generator.
    fn reset(&mut self);
}

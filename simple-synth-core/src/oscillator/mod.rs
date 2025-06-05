mod basic;
mod simd;

// TODO: wavetables
// TODO: anti-aliasing (e.g. PolyBLEP)
pub use basic::BasicOscillator;
pub use simd::SimdOscillator;

/// Represents the different waveform shapes an oscillator can generate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

/// Trait defining an oscillator.
pub trait Oscillator {
    /// The output sample type produced by the oscillator.
    type Output;

    /// Produces the next sample (or block of samples) from the oscillator.
    ///
    /// Returns a value of type `Output`.
    fn tick(&mut self) -> Self::Output;
    /// Sets the oscillator frequency in hertz.
    fn set_frequency(&mut self, frequency: f32);
    /// Sets the waveform shape to generate, based on the [`Waveform`] enum.
    fn set_waveform(&mut self, waveform: Waveform);
    /// Sets the sample rate in hertz.
    fn set_sample_rate(&mut self, sample_rate: f32);
    /// Resets the internal state of the oscillator.
    fn reset(&mut self);
}

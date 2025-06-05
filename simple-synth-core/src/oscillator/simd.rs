use std::f32::consts::PI;
use rand::random;
use wide::{f32x4, CmpLt};
use super::{Oscillator, Waveform};

pub struct SimdOscillator {
    phase: f32x4,
    step: f32,
    sample_rate: f32,
    frequency: f32,
    waveform: Waveform,
}

impl SimdOscillator {
    pub fn new(sample_rate: f32) -> Self {
        assert!(sample_rate > 0.0, "Sample rate must be greater than 0.");

        let frequency = 440.0;
        let step = frequency / sample_rate;

        Self {
            phase: f32x4::from([0.0, step, 2.0 * step, 3.0 * step]),
            step,
            sample_rate,
            frequency,
            waveform: Waveform::Sine,
        }
    }

    fn generate_sine(&self) -> f32x4 {
        (f32x4::splat(2.0 * PI) * self.phase).sin()
    }

    fn generate_square(&self) -> f32x4 {
        self.phase
            .cmp_lt(0.5)
            .blend(f32x4::splat(1.0), f32x4::splat(-1.0))
    }

    fn generate_triangle(&self) -> f32x4 {
        self.phase
            .cmp_lt(0.5)
            .blend(
                f32x4::splat(4.0) * self.phase - f32x4::splat(1.0),
                f32x4::splat(3.0) - f32x4::splat(4.0) * self.phase
            )
    }

    fn generate_sawtooth(&self) -> f32x4 {
        f32x4::splat(2.0) * self.phase - f32x4::splat(1.0)
    }

    /// TODO: simd rng
    fn generate_noise(&self) -> f32x4 {
        f32x4::from([
            2.0 * random::<f32>() - 1.0,
            2.0 * random::<f32>() - 1.0,
            2.0 * random::<f32>() - 1.0,
            2.0 * random::<f32>() - 1.0
        ])
    }
}

impl Oscillator for SimdOscillator {
    type Output = f32x4;

    /// Generate the next block of 4 samples for one voice.
    fn tick(&mut self) -> f32x4 {
        let sample = match self.waveform {
            Waveform::Sine => self.generate_sine(),
            Waveform::Square => self.generate_square(),
            Waveform::Triangle => self.generate_triangle(),
            Waveform::Sawtooth => self.generate_sawtooth(),
            Waveform::Noise => self.generate_noise(),
        };

        // self.phase = self.phase + f32x4::splat(4.0 * self.step);
        self.phase += f32x4::splat(4.0 * self.step);
        // TODO: rem_euclid
        self.phase = self.phase - self.phase.floor();

        sample
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.clamp(0.0, self.sample_rate / 2.0);
        self.step = self.frequency / self.sample_rate;
    }

    fn set_waveform(&mut self, waveform: Waveform) {
        if self.waveform != waveform {
            self.waveform = waveform;
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        if sample_rate > 0.0 {
            self.sample_rate = sample_rate;
            self.step = self.frequency / self.sample_rate;
        }
    }

    fn reset(&mut self) {
        self.phase = f32x4::from([0.0, self.step, 2.0 * self.step, 3.0 * self.step]);
    }
}

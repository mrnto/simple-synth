use std::f32::consts::PI;
use rand::random;
use nih_plug::prelude::Enum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Square"]
    Square,
    #[name = "Triangle"]
    Triangle,
    #[name = "Sawtooth"]
    Sawtooth,
    #[name = "Noise"]
    Noise,
}

// TODO: wavetables
// TODO: anti-aliasing (e.g. PolyBLEP)
pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    frequency: f32,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            frequency: 440.0,
            waveform: Waveform::Sine,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => self.generate_sine(),
            Waveform::Square => self.generate_square(),
            Waveform::Triangle => self.generate_triangle(),
            Waveform::Sawtooth => self.generate_sawtooth(),
            Waveform::Noise => self.generate_noise(),
        };

        self.phase = (self.phase + self.frequency / self.sample_rate).rem_euclid(1.0);

        sample
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.clamp(0.0, self.sample_rate / 2.0);
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        if self.waveform != waveform {
            self.waveform = waveform;
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        if sample_rate > 0.0 {
            self.sample_rate = sample_rate;
        }
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    fn generate_sine(&self) -> f32 {
        (2.0 * PI * self.phase).sin()
    }

    fn generate_square(&self) -> f32 {
        if self.phase < 0.5 {
            1.0
        } else {
            -1.0
        }
    }

    fn generate_triangle(&self) -> f32 {
        if self.phase < 0.5 {
            4.0 * self.phase - 1.0
        } else {
            3.0 - 4.0 * self.phase
        }
    }

    fn generate_sawtooth(&self) -> f32 {
        2.0 * self.phase - 1.0
    }

    fn generate_noise(&self) -> f32 {
        2.0 * random::<f32>() - 1.0
    }
}

use rand::random;
use std::f32::consts::PI;

#[derive(PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

pub struct Oscillator {
    frequency: f32,
    phase: f32,
    sample_rate: u32,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            frequency: 440.0,
            phase: 0.0,
            sample_rate,
            waveform: Waveform::Sine,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => self.generate_sine(),
            Waveform::Square => self.generate_square(),
            Waveform::Triangle => self.generate_triangle(),
            Waveform::Sawtooth => self.generate_sawtooth(),
            Waveform::Noise =>
                if self.frequency == 0.0 { 0.0 } else { self.generate_noise() },
        };

        self.phase = (self.phase + self.frequency / self.sample_rate as f32).rem_euclid(1.0);

        sample
    }

    // TODO: Fix "pops" when reset
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.clamp(0.0, self.sample_rate as f32 / 2.0);
        // self.phase = 0.0;
    }

    // TODO: Fix "pops" when reset
    pub fn set_waveform(&mut self, waveform: Waveform) {
        if self.waveform != waveform {
            self.waveform = waveform;
            // self.phase = 0.0;
        }
    }

    // TODO
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        if sample_rate <= 0 {
            return;
        }
        
        self.sample_rate = sample_rate;
        self.frequency = 0.0;
    }

    fn generate_sine(&self) -> f32 {
        (2.0 * PI * self.phase).sin()
    }

    fn generate_square(&self) -> f32 {
        if self.phase < 0.5 { 1.0 } else { -1.0 }
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
        random::<f32>() * 2.0 - 1.0
    }
}

use rand::random;
use std::f32::consts::PI;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

impl FromStr for Waveform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sine" => Ok(Self::Sine),
            "Square" => Ok(Self::Square),
            "Triangle" => Ok(Self::Triangle),
            "Sawtooth" => Ok(Self::Sawtooth),
            "Noise" => Ok(Self::Noise),
            _ => Err(()),
        }
    }
}

// TODO: implement wavetables
pub struct Oscillator {
    sample_rate: u32,
    phase: f32,
    frequency: f32,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(sample_rate: u32) -> Self {
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
            Waveform::Noise =>
                if self.frequency == 0.0 { 0.0 } else { self.generate_noise() },
        };

        self.phase = (self.phase + self.frequency / self.sample_rate as f32).rem_euclid(1.0);

        sample
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.clamp(0.0, self.sample_rate as f32 / 2.0);
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        if self.waveform != waveform {
            self.waveform = waveform;
        }
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
        2.0 * random::<f32>() - 1.0
    }
}

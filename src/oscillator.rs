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
    waveform: Waveform,
    frequency: f32,
    sample_rate: u32,
    phase: f32,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f32, sample_rate: u32) -> Self {
        Self {
            waveform,
            frequency,
            sample_rate,
            phase: 0.0,
        }
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        if self.waveform != waveform {
            self.phase = 0.0;
        }

        self.waveform = waveform;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    pub fn tick(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => self.generate_sine(),
            Waveform::Square => self.generate_square(),
            Waveform::Triangle => self.generate_triangle(),
            Waveform::Sawtooth => self.generate_sawtooth(),
            Waveform::Noise => {
                if self.frequency == 0.0 {
                    0.0
                } else {
                    self.generate_noise()
                }
            },
        };

        // self.phase += self.frequency / self.sample_rate as f32;
        // if self.phase >= 1.0 {
        //     self.phase -= 1.0;
        // }
        self.phase = (self.phase + self.frequency / self.sample_rate as f32).rem_euclid(1.0);

        sample
    }

    fn generate_sine(&self) -> f32 {
        (2.0 * PI * self.phase).sin()
    }

    fn generate_square(&self) -> f32 {
        // if self.generate_sine() <= 0.0 { 1.0 } else { -1.0 }
        if self.phase < 0.5 { 1.0 } else { -1.0 }
    }

    fn generate_triangle(&self) -> f32 {
        // 2.0 / PI * self.generate_sine().asin()
        if self.phase < 0.5 {
            4.0 * self.phase - 1.0
        } else {
            3.0 - 4.0 * self.phase
        }
    }

    fn generate_sawtooth(&self) -> f32 {
        // -2.0 / PI * (PI * self.phase).tan().atan()
        2.0 * self.phase - 1.0
    }

    fn generate_noise(&self) -> f32 {
        random::<f32>() * 2.0 - 1.0
    }
}

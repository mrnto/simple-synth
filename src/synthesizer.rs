use crate::oscillator::{Oscillator, Waveform};

pub struct Synthesizer {
    // oscillators: Vec<Oscillator>,
    oscillator: Option<Oscillator>,
}

impl Synthesizer {
    pub fn new() -> Self {
        Self {
            oscillator: None,
        }
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        if let Some(ref mut oscillator) = self.oscillator {
            oscillator.set_waveform(waveform);
        } else {
            eprintln!("Oscillator is not set up.");
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        if let Some(ref mut oscillator) = self.oscillator {
            oscillator.set_frequency(frequency);
        } else {
            eprintln!("Oscillator is not set up.");
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        if let Some(ref mut oscillator) = self.oscillator {
            oscillator.set_sample_rate(sample_rate);
        } else {
            eprintln!("Oscillator is not set up.");
        }
    }

    pub fn set_oscillator(&mut self, waveform: Waveform, frequency: f32, sample_rate: u32) {
        self.oscillator = Some(Oscillator::new(waveform, frequency, sample_rate));
    }

    pub fn generate(&mut self) -> f32 {
        self.oscillator.as_mut().map_or(0.0, |osc| osc.tick())
    }
}

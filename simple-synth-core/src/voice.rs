use crate::{
    envelope::{Envelope, LinearEnvelope},
    EnvelopeStage,
    filter::Filter,
    oscillator::{BasicOscillator, Oscillator},
    SynthParam
};

pub struct Voice {
    oscillator1: BasicOscillator,
    // oscillator2: Oscillator,
    envelope1: LinearEnvelope,
    // envelope2: Envelope,
    filter: Filter,
    note_number: Option<u8>,
    // velocity: Option<u8>,
    active: bool,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillator1: BasicOscillator::new(sample_rate),
            // oscillator2: Oscillator::new(sample_rate),
            envelope1: LinearEnvelope::new(sample_rate),
            // envelope2: Envelope::new(sample_rate),
            filter: Filter::new(),
            note_number: None,
            // velocity: None,
            active: false,
        }
    }

    pub fn process(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }

        if self.envelope1.is_idle() {
            self.active = false;
            self.note_number = None;
            return 0.0;
        }

        self.filter.process(self.oscillator1.tick() * self.envelope1.process())
        // self.oscillator1.tick() * self.envelope1.process() * (self.velocity / 127.0)
    }

    pub fn note_on(&mut self, note_number: u8) {
        if !self.active {
            let frequency = self.midi_note_to_frequency(note_number);

            self.oscillator1.set_frequency(frequency);
            self.envelope1.trigger();
            self.note_number = Some(note_number);
            self.active = true;
        } else {
            self.envelope1.trigger();
        }
    }

    pub fn note_off(&mut self, _note_number: u8) {
        if self.active {
            self.envelope1.release();
        }
    }

    pub fn apply_param(&mut self, param: SynthParam) {
        match param {
            SynthParam::EnvelopeStage(stage, value) => {
                match stage {
                    EnvelopeStage::Attack => self.envelope1.set_attack_time(value),
                    EnvelopeStage::Decay => self.envelope1.set_decay_time(value),
                    EnvelopeStage::Sustain => self.envelope1.set_sustain_level(value),
                    EnvelopeStage::Release => self.envelope1.set_release_time(value),
                    EnvelopeStage::Idle => (),
                }
            },
            SynthParam::Waveform(waveform) => self.oscillator1.set_waveform(waveform),
            SynthParam::FilterMode(mode) => self.filter.set_mode(mode),
            SynthParam::Cutoff(value) => self.filter.set_cutoff(value),
            SynthParam::Resonance(value) => self.filter.set_resonance(value),
            SynthParam::SampleRate(rate) => {
                self.oscillator1.set_sample_rate(rate);
                self.envelope1.set_sample_rate(rate);
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn note_number(&self) -> Option<u8> {
        self.note_number
    }

    pub fn reset(&mut self) {
        self.note_number = None;
        self.active = false;
        self.oscillator1.reset();
        self.envelope1.reset();
        self.filter.reset();
    }

    fn midi_note_to_frequency(&self, note_number: u8) -> f32 {
        440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0)
    }
}

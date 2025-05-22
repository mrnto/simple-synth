use crate::synthesizer::{
    envelope::Envelope,
    filter::Filter,
    oscillator::Oscillator,
    SynthParam,
};

pub struct Voice {
    oscillator1: Oscillator,
    // oscillator2: Oscillator,
    envelope1: Envelope,
    // envelope2: Envelope,
    filter: Filter,
    note_number: Option<u8>,
    // velocity: Option<u8>,
    active: bool,
}

impl Voice {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            oscillator1: Oscillator::new(sample_rate),
            // oscillator2: Oscillator::new(sample_rate),
            envelope1: Envelope::new(sample_rate),
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
            match self.note_number {
                Some(n) => println!("[debug] Free note {}.", n),
                None => println!("[debug] Free note <none>."),
            };
            self.note_number = None;
            return 0.0;
        }

        self.filter.process(self.oscillator1.tick() * self.envelope1.process())
        // self.oscillator1.tick() * self.envelope1.process() * (self.velocity / 127.0)
    }

    pub fn note_on(&mut self, note_number: u8) {
        if !self.active {
            println!("[debug] Start attack for note {}.", note_number);
            let frequency = self.midi_note_to_frequency(note_number);

            self.oscillator1.set_frequency(frequency);
            self.envelope1.start_attack();
            self.note_number = Some(note_number);
            self.active = true;
        } else {
            println!("[debug] Retrigger note {}.", note_number);
            self.envelope1.start_attack();
        }
    }

    pub fn note_off(&mut self, note_number: u8) {
        if self.active {
            println!("[debug] Start release for note {}.", note_number);
            self.envelope1.start_release();
        }
    }

    pub fn apply_param(&mut self, param: SynthParam) {
        match param {
            SynthParam::EnvelopeStage(stage, value) => self.envelope1.set_stage_value(stage, value),
            SynthParam::Waveform(waveform) => self.oscillator1.set_waveform(waveform),
            SynthParam::FilterMode(mode) => self.filter.set_mode(mode),
            SynthParam::Cutoff(value) => self.filter.set_cutoff(value),
            SynthParam::Resonance(value) => self.filter.set_resonance(value),
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn note_number(&self) -> Option<u8> {
        self.note_number
    }

    fn midi_note_to_frequency(&self, note_number: u8) -> f32 {
        440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0)
    }
}

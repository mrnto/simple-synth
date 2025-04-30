use crate::{
    synthesizer::envelope::Envelope,
    synthesizer::oscillator::Oscillator,
    synthesizer::Waveform,
};

pub struct Voice {
    oscillator1: Oscillator,
    // oscillator2: Oscillator,
    envelope1: Envelope,
    // envelope2: Envelope,
    note_number: Option<u8>,
    // velocity: u8,
    active: bool,
}

impl Voice {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            oscillator1: Oscillator::new(sample_rate),
            // oscillator2: Oscillator::new(sample_rate),
            envelope1: Envelope::new(sample_rate),
            // envelope2: Envelope::new(sample_rate),
            note_number: None,
            active: false,
        }
    }

    pub fn process(&mut self) -> f32 {
        if !self.active { return 0.0; }

        if self.envelope1.is_idle() == true {
            self.free_voice();
            self.note_number = None;
            println!("free note");
        }

        self.oscillator1.tick() * self.envelope1.process()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    // tmp
    pub fn get_note(&self) -> Option<u8> {
        self.note_number
    }

    // TODO
    pub fn note_on(&mut self, note_number: u8) {
        self.oscillator1.set_frequency(self.midi_note_to_frequency(note_number));
        self.envelope1.start_attack();
        self.active = true;
        self.note_number = Some(note_number);
    }
    
    pub fn note_off(&mut self, note_number: u8) {
        self.envelope1.start_release();
    }
    
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator1.set_frequency(frequency);
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.oscillator1.set_waveform(waveform);
    }

    pub fn start_attack(&mut self) {
        self.envelope1.start_attack();
    }

    pub fn start_release(&mut self) {
        self.envelope1.start_release();
    }

    pub fn set_attack(&mut self, attack: f32) {
        self.envelope1.set_attack(attack);
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.envelope1.set_decay(decay);
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.envelope1.set_sustain(sustain);
    }

    pub fn set_release(&mut self, release: f32) {
        self.envelope1.set_release(release);
    }

    fn free_voice(&mut self) {
        self.active = false;
    }

    fn midi_note_to_frequency(&self, note_number: u8) -> f32 {
        440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0)
    }
}

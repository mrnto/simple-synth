use std::array::from_fn;
use crate::synthesizer::{SynthParam, voice::Voice};

const MAX_VOICES: usize = 16;

// TODO: reduce redundant checks (HashMap, VoicePool, etc.)
pub struct VoiceManager {
    voices: [Voice; MAX_VOICES],
}

impl VoiceManager {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voices: from_fn(|_| Voice::new(sample_rate)),
        }
    }

    pub fn process_voices(&mut self) -> f32 {
        self.voices.iter_mut().map(|v| v.process()).sum()
        // let mixed = self.voices.iter_mut().map(|v| v.process()).sum::<f32>();
        // mixed * 0.5 // TODO: add master volume, gain
    }

    pub fn note_on(&mut self, note_number: u8) {
        if let Some(existing_voice) = self.find_voice_by_note(note_number) {
            existing_voice.note_on(note_number);
        } else if let Some(free_voice) = self.find_free_voice() {
            free_voice.note_on(note_number);
        }
    }

    pub fn note_off(&mut self, note_number: u8) {
        if let Some(voice) = self.find_voice_by_note(note_number) {
            voice.note_off(note_number);
        }
    }

    pub fn apply_param(&mut self, param: SynthParam) {
        for voice in &mut self.voices {
            voice.apply_param(param);
        }
    }

    fn find_free_voice(&mut self) -> Option<&mut Voice> {
        self.voices.iter_mut().find(|v| !v.active())
    }

    fn find_voice_by_note(&mut self, note_number: u8) -> Option<&mut Voice> {
        self.voices
            .iter_mut()
            .find(|v| v.active() && v.note_number() == Some(note_number))
    }
}

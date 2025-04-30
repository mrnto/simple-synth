use crate::{voice::Voice, waveform::Waveform};

const MAX_VOICES: usize = 16;

// TODO: fixed-size array
pub struct VoiceManager {
    voices: Vec<Voice>,
}

impl VoiceManager {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            voices: (0..MAX_VOICES).map(|_| Voice::new(sample_rate)).collect(),
        }
    }

    pub fn process_voices(&mut self) -> f32 {
        self.voices.iter_mut().map(|v| v.process()).sum()
    }

    pub fn note_on(&mut self, note_number: u8) {
        if let Some(free_voice) = self.find_free_voice() {
            free_voice.note_on(note_number);
        }
    }

    pub fn note_off(&mut self, note_number: u8) {
        if let Some(voice) = self.find_voice_by_note(note_number) {
            voice.note_off(note_number);
        }
    }

    // meh...
    pub fn set_frequency(&mut self, frequency: f32) {
        for voice in &mut self.voices {
            voice.set_frequency(frequency);
        }
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        for voice in &mut self.voices {
            voice.set_waveform(waveform);
        }
    }

    pub fn set_attack(&mut self, attack: f32) {
        for voice in &mut self.voices {
            voice.set_attack(attack);
        }
    }

    pub fn set_decay(&mut self, decay: f32) {
        for voice in &mut self.voices {
            voice.set_decay(decay);
        }
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        for voice in &mut self.voices {
            voice.set_sustain(sustain);
        }
    }

    pub fn set_release(&mut self, release: f32) {
        for voice in &mut self.voices {
            voice.set_release(release);
        }
    }

    fn find_free_voice(&mut self) -> Option<&mut Voice> {
        self.voices.iter_mut().find(|v| !v.is_active())
    }

    fn find_voice_by_note(&mut self, note_number: u8) -> Option<&mut Voice> {
        self.voices.iter_mut().find(|v| v.is_active() && v.get_note() == Some(note_number))
    }
}

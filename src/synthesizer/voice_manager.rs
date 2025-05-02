use crate::{
    synthesizer::EnvelopeStage,
    synthesizer::voice::Voice,
    synthesizer::Waveform
};

const MAX_VOICES: usize = 16;

// TODO: fixed-size array
// TODO: reduce redundant checks (HashMap, VoicePool, etc.)
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

    pub fn set_waveform(&mut self, waveform: Waveform) {
        for voice in &mut self.voices {
            voice.set_waveform(waveform);
        }
    }

    pub fn set_stage_value(&mut self, stage: EnvelopeStage, value: f32) {
        for voice in &mut self.voices {
            voice.set_stage_value(stage, value);
        }
    }

    fn find_free_voice(&mut self) -> Option<&mut Voice> {
        self.voices.iter_mut().find(|v| !v.active())
    }

    fn find_voice_by_note(&mut self, note_number: u8) -> Option<&mut Voice> {
        self.voices.iter_mut().find(|v| v.active() && v.note_number() == Some(note_number))
    }
}

use crate::{
    messages::SynthMsg,
    synthesizer::voice_manager::VoiceManager,
};

pub struct Synthesizer {
    voice_manager: VoiceManager,
}

impl Synthesizer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            voice_manager: VoiceManager::new(sample_rate),
        }
    }

    pub fn generate(&mut self) -> f32 {
        self.voice_manager.process_voices()
    }

    pub fn handle_message(&mut self, message: SynthMsg) {
        match message {
            SynthMsg::NoteOn(note_number) => self.voice_manager.note_on(note_number),
            SynthMsg::NoteOff(note_number) => self.voice_manager.note_off(note_number),
            SynthMsg::SetStage(stage, value) => self.voice_manager.set_stage_value(stage, value),
            SynthMsg::SetWaveform(waveform) => self.voice_manager.set_waveform(waveform),
        }
    }
}

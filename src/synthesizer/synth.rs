use crate::{commands::SynthCommand, synthesizer::voice_manager::VoiceManager};

pub struct Synth {
    voice_manager: VoiceManager,
}

impl Synth {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            voice_manager: VoiceManager::new(sample_rate),
        }
    }

    pub fn generate(&mut self) -> f32 {
        self.voice_manager.process_voices()
    }

    pub fn handle_command(&mut self, command: SynthCommand) {
        match command {
            SynthCommand::NoteOn(midi_note_number) => self.voice_manager.note_on(midi_note_number),
            SynthCommand::NoteOff(midi_note_number) => self.voice_manager.note_off(midi_note_number),
            SynthCommand::SetParam(param) => self.voice_manager.apply_param(param),
        }
    }
}

use crate::{
    messages::{EnvelopeMsg, OscillatorMsg, SynthMsg},
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
            SynthMsg::EnvelopeMsg(env_msg) => {
                match env_msg {
                    EnvelopeMsg::SetAttack(attack) => self.voice_manager.set_attack(attack),
                    EnvelopeMsg::SetDecay(decay) => self.voice_manager.set_decay(decay),
                    EnvelopeMsg::SetSustain(sustain) => self.voice_manager.set_sustain(sustain),
                    EnvelopeMsg::SetRelease(release) => self.voice_manager.set_release(release),
                }
            },
            SynthMsg::OscillatorMsg(osc_msg) => {
                match osc_msg {
                    OscillatorMsg::NoteOn(frequency) => {
                        self.voice_manager.note_on(frequency);
                    },
                    OscillatorMsg::NoteOff(frequency) => {
                        self.voice_manager.note_off(frequency);
                    },
                    OscillatorMsg::SetWaveform(waveform) => 
                    {
                        self.voice_manager.set_waveform(waveform);
                    },
                    OscillatorMsg::SetOscillator(sample_rate, waveform, frequency) => {
                        // 
                    },
                }
            },
        }
    }
}

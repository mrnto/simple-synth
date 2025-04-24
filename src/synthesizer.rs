use crate::{
    envelope::Envelope,
    messages::{EnvelopeMsg, OscillatorMsg, SynthMsg},
    oscillator::{Oscillator, Waveform}
};

pub struct Synthesizer {
    // oscillators: Vec<Option<Oscillator>>,
    // envelopes: Vec<Envelope>,
    oscillator: Option<Oscillator>,
    envelope: Envelope,
}

impl Synthesizer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            oscillator: Some(Oscillator::new(sample_rate)),
            envelope: Envelope::new(sample_rate),
        }
    }

    pub fn generate(&mut self) -> f32 {
        self.oscillator.as_mut().map_or(0.0, |osc| osc.tick() * self.envelope.process())
    }

    pub fn handle_message(&mut self, message: SynthMsg) {
        match message {
            SynthMsg::EnvelopeMsg(env_msg) => {
                match env_msg {
                    EnvelopeMsg::SetAttack(attack) => self.envelope.set_attack(attack),
                    EnvelopeMsg::SetDecay(decay) => self.envelope.set_decay(decay),
                    EnvelopeMsg::SetSustain(sustain) => self.envelope.set_sustain(sustain),
                    EnvelopeMsg::SetRelease(release) => self.envelope.set_release(release),
                }
            },
            SynthMsg::OscillatorMsg(osc_msg) => {
                match osc_msg {
                    OscillatorMsg::SetFrequency(frequency) => {
                        if let Some(ref mut osc) = self.oscillator {
                            if frequency == 0.0 {   
                                self.envelope.start_release();
                            } else {
                                self.envelope.start_attack();
                                osc.set_frequency(frequency);
                            }
                        }
                    },
                    OscillatorMsg::SetWaveform(waveform) => 
                    {
                        if let Some(ref mut osc) = self.oscillator {
                            osc.set_waveform(waveform)
                        }
                    },
                    OscillatorMsg::SetSampleRate(sample_rate) => {
                        if let Some(ref mut osc) = self.oscillator {
                            // osc.set_sample_rate(sample_rate)
                        }
                    },
                    OscillatorMsg::SetOscillator(sample_rate, waveform, frequency) => {
                        self.oscillator = Some(Oscillator::new(sample_rate))
                        // then set
                    },
                }
            },
        }
    }
}

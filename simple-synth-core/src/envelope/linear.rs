use super::{Envelope, EnvelopeStage};

// TODO: adsr change when note is played
pub struct LinearEnvelope {
    sample_rate: f32,
    attack_rate: f32,
    decay_rate: f32,
    release_rate: f32,
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
    level: f32,
    stage: EnvelopeStage,
}

impl LinearEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            attack_rate: 0.1 * sample_rate,
            decay_rate: 0.2 * sample_rate,
            release_rate: 1.0 * sample_rate,
            attack_time: 0.1,
            decay_time: 0.2,
            sustain_level: 0.8,
            release_time: 1.0,
            level: 0.0,
            stage: EnvelopeStage::Idle,
        }
    }

    fn set_stage_value(&mut self, stage: EnvelopeStage, value: f32) {
        match stage {
            EnvelopeStage::Idle => (),
            EnvelopeStage::Attack => {
                self.attack_time = value.clamp(0.000001, 10.0);
                self.attack_rate = self.attack_time * self.sample_rate;
            },
            EnvelopeStage::Decay => {
                self.decay_time = value.clamp(0.000001, 10.0);
                self.decay_rate = self.decay_time * self.sample_rate;
            },
            EnvelopeStage::Sustain => {
                self.sustain_level = value.clamp(0.0, 1.0);
            },
            EnvelopeStage::Release => {
                self.release_time = value.clamp(0.000001, 10.0);
                self.release_rate = self.release_time * self.sample_rate;
            },
        }
    }
}

impl Envelope for LinearEnvelope {
    type Output = f32;

    fn process(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Idle | EnvelopeStage::Sustain => {},
            EnvelopeStage::Attack => {
                self.level += 1.0 / self.attack_rate;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }
            },
            EnvelopeStage::Decay => {
                self.level -= 1.0 / self.decay_rate;
                if self.level <= self.sustain_level {
                    self.level = self.sustain_level;
                    self.stage = EnvelopeStage::Sustain;
                }
            },
            EnvelopeStage::Release => {
                self.level -= 1.0 / self.release_rate;
                if self.level <= 0.0 {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            },
        }

        self.level
    }

    fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
    }

    fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    fn is_idle(&self) -> bool {
        self.stage == EnvelopeStage::Idle
    }

    fn set_attack_time(&mut self, attack_time: f32) {
        self.set_stage_value(EnvelopeStage::Attack, attack_time);
    }

    fn set_decay_time(&mut self, decay_time: f32) {
        self.set_stage_value(EnvelopeStage::Decay, decay_time);
    }

    fn set_sustain_level(&mut self, sustain_level: f32) {
        self.set_stage_value(EnvelopeStage::Sustain, sustain_level);
    }

    fn set_release_time(&mut self, release_time: f32) {
        self.set_stage_value(EnvelopeStage::Release, release_time);
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        if sample_rate > 0.0 {
            self.sample_rate = sample_rate;
            self.attack_rate = self.attack_time * self.sample_rate;
            self.decay_rate = self.decay_time * self.sample_rate;
            self.release_rate = self.release_time * self.sample_rate;
        }
    }

    fn reset(&mut self) {
        self.stage = EnvelopeStage::Idle;
        self.level = 0.0;
    }
}

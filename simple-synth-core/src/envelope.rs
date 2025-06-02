#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

// TODO: exponential
// TODO: adsr change when note is played
pub struct Envelope {
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

impl Envelope {
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

    pub fn process(&mut self) -> f32 {
        match self.stage {
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
            EnvelopeStage::Sustain => {},
            EnvelopeStage::Release => {
                self.level -= 1.0 / self.release_rate;
                if self.level <= 0.0 {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            },
            EnvelopeStage::Idle => {},
        }
        
        self.level
    }

    pub fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
    }

    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    pub fn set_stage_value(&mut self, stage: EnvelopeStage, value: f32) {
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

    pub fn is_idle(&self) -> bool {
        self.stage == EnvelopeStage::Idle
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        if sample_rate > 0.0 {
            self.sample_rate = sample_rate;
            self.attack_rate = self.attack_time * self.sample_rate;
            self.decay_rate = self.decay_time * self.sample_rate;
            self.release_rate = self.release_time * self.sample_rate;
        }
    }

    pub fn reset(&mut self) {
        self.stage = EnvelopeStage::Idle;
        self.level = 0.0;
    }
}

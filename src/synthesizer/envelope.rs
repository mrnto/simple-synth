const DEFAULT_ATTACK: f32 = 0.2;
const DEFAULT_DECAY: f32 = 0.2;
const DEFAULT_SUSTAIN: f32 = 0.8;
const DEFAULT_RELEASE: f32 = 0.2;

const MAX_ATTACK_TIME: f32 = 10.0;
const MAX_DECAY_TIME: f32 = 10.0;
const MAX_SUSTAIN_LEVEL: f32 = 1.0;
const MAX_RELEASE_TIME: f32 = 10.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Idle,
}

// TODO: exponential; adsr change when note is played
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
            attack_rate: DEFAULT_ATTACK * sample_rate,
            decay_rate: DEFAULT_DECAY * sample_rate,
            release_rate: DEFAULT_RELEASE * sample_rate,
            attack_time: DEFAULT_ATTACK,
            decay_time: DEFAULT_DECAY,
            sustain_level: DEFAULT_SUSTAIN,
            release_time: DEFAULT_RELEASE,
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
                if self.level <= f32::EPSILON {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            },
            EnvelopeStage::Idle => {},
        }
        
        self.level
    }

    pub fn start_attack(&mut self) {
        self.stage = EnvelopeStage::Attack;
    }

    pub fn start_release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    pub fn set_stage_value(&mut self, stage: EnvelopeStage, value: f32) {
        match stage {
            EnvelopeStage::Attack => {
                self.attack_time = value.clamp(0.000001, MAX_ATTACK_TIME);
                self.attack_rate = self.attack_time * self.sample_rate;
            },
            EnvelopeStage::Decay => {
                self.decay_time = value.clamp(0.000001, MAX_DECAY_TIME);
                self.decay_rate = self.decay_time * self.sample_rate;
            },
            EnvelopeStage::Sustain => {
                self.sustain_level = value.clamp(0.0, MAX_SUSTAIN_LEVEL);
            },
            EnvelopeStage::Release => {
                self.release_time = value.clamp(0.000001, MAX_RELEASE_TIME);
                self.release_rate = self.release_time * self.sample_rate;
            },
            EnvelopeStage::Idle => (), 
        }
    }

    pub fn is_idle(&self) -> bool {
        self.stage == EnvelopeStage::Idle
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.attack_rate = self.attack_time * self.sample_rate;
        self.decay_rate = self.decay_time * self.sample_rate;
        self.release_rate = self.release_time * self.sample_rate;
    }
}

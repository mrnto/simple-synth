const DEFAULT_ATTACK: f32 = 0.2;
const DEFAULT_DECAY: f32 = 0.2;
const DEFAULT_SUSTAIN: f32 = 0.8;
const DEFAULT_RELEASE: f32 = 0.2;

const MAX_ATTACK_TIME: f32 = 10.0;
const MAX_DECAY_TIME: f32 = 10.0;
const MAX_SUSTAIN_LEVEL: f32 = 1.0;
const MAX_RELEASE_TIME: f32 = 10.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Idle,
}

// TODO: exponential; adsr change when note is played
pub struct Envelope {
    sample_rate: u32,
    attack_rate: u32,
    decay_rate: u32,
    release_rate: u32,
    sustain_level: f32,
    level: f32,
    stage: EnvelopeStage,
}

impl Envelope {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            attack_rate: (DEFAULT_ATTACK * sample_rate as f32).round() as u32,
            decay_rate: (DEFAULT_DECAY * sample_rate as f32).round() as u32,
            sustain_level: DEFAULT_SUSTAIN,
            release_rate: (DEFAULT_RELEASE * sample_rate as f32).round() as u32,
            level: 0.0,
            stage: EnvelopeStage::Idle,
        }
    }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Attack => {
                self.level += 1.0 / self.attack_rate as f32;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }
            },
            EnvelopeStage::Decay => {
                self.level -= 1.0 / self.decay_rate as f32;
                if self.level <= self.sustain_level {
                    self.level = self.sustain_level;
                    self.stage = EnvelopeStage::Sustain;
                }
            },
            EnvelopeStage::Sustain => {},
            EnvelopeStage::Release => {
                self.level -= 1.0 / self.release_rate as f32;
                if self.level <= 0.0 {
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
                let attack = value.clamp(0.0, MAX_ATTACK_TIME);
                self.attack_rate = (attack * self.sample_rate as f32).round() as u32;
            },
            EnvelopeStage::Decay => {
                let decay = value.clamp(0.0, MAX_DECAY_TIME);
                self.decay_rate = (decay * self.sample_rate as f32).round() as u32;
            },
            EnvelopeStage::Sustain => {
                self.sustain_level = value.clamp(0.0, MAX_SUSTAIN_LEVEL);
            },
            EnvelopeStage::Release => {
                let release = value.clamp(0.0, MAX_RELEASE_TIME);
                self.release_rate = (release * self.sample_rate as f32).round() as u32;
            },
            EnvelopeStage::Idle => (), 
        }
    }

    pub fn is_idle(&self) -> bool {
        self.stage == EnvelopeStage::Idle
    }
}

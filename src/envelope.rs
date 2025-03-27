const DEFAULT_ATTACK: f32 = 0.2;
const DEFAULT_DECAY: f32 = 0.2;
const DEFAULT_SUSTAIN: f32 = 0.8;
const DEFAULT_RELEASE: f32 = 0.2;

const MAX_ATTACK_TIME: f32 = 10.0;
const MAX_DECAY_TIME: f32 = 10.0;
const MAX_SUSTAIN_LEVEL: f32 = 1.0;
const MAX_RELEASE_TIME: f32 = 10.0;

#[derive(PartialEq)]
enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
    Idle,
}

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    delta_time: f32,
    time: f32,
    state: EnvelopeState,
}

impl Envelope {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            attack: DEFAULT_ATTACK,
            decay: DEFAULT_DECAY,
            sustain: DEFAULT_SUSTAIN,
            release: DEFAULT_RELEASE,
            delta_time: 1.0 / sample_rate as f32,
            time: 0.0,
            state: EnvelopeState::Idle,
        }
    }

    pub fn process(&mut self) -> f32 {
        self.time += self.delta_time;

        match self.state {
            EnvelopeState::Attack => {
                if self.time >= self.attack {
                    self.state = EnvelopeState::Decay;
                }
                
                self.time / self.attack
            },
            EnvelopeState::Decay => {
                if self.time >= (self.attack + self.decay) {
                    self.state = EnvelopeState::Sustain;
                }
                
                1.0 - ((self.time - self.attack) / self.decay) * (1.0 - self.sustain)
            },
            EnvelopeState::Sustain => self.sustain,
            EnvelopeState::Release => {
                if self.time >= self.release {
                    self.state = EnvelopeState::Idle;
                }

                // self.sustain * (1.0 - self.time / self.release)
                // TODO: FIX "pops" on fast release
                self.sustain * (1.0 - (self.time / self.release).powf(2.0))
            },
            EnvelopeState::Idle => 0.0,
        }
    }

    pub fn start_attack(&mut self) {
        self.state = EnvelopeState::Attack;
        self.time = 0.0;
    }

    pub fn start_release(&mut self) {
        self.state = EnvelopeState::Release;
        self.time = 0.0;
    }

    pub fn get_adsr(&self) -> (f32, f32, f32, f32) {
        (self.attack, self.decay, self.sustain, self.release)
    }

    pub fn set_attack(&mut self, attack: f32) {
        self.attack = attack.clamp(0.0, MAX_ATTACK_TIME);
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay.clamp(0.0, MAX_DECAY_TIME);
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.sustain = sustain.clamp(0.0, MAX_SUSTAIN_LEVEL);
    }

    pub fn set_release(&mut self, release: f32) {
        self.release = release.clamp(0.0, MAX_RELEASE_TIME);
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        if sample_rate <= 0 {
            return;
        }

        self.delta_time = 1.0 / sample_rate as f32;
    }
}

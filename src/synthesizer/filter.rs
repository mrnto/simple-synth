use nih_plug::prelude::Enum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum FilterMode {
    #[name = "Lowpass"]
    Lowpass,
    #[name = "Highpass"]
    Highpass,
    #[name = "Bandpass"]
    Bandpass,
}

pub struct Filter {
    cutoff: f32,
    resonance: f32,
    feedback: f32,
    buf0: f32,
    buf1: f32,
    mode: FilterMode,
}

impl Filter {
    pub fn new() -> Self {
        let mut filter = Self {
            cutoff: 0.99,
            resonance: 0.0,
            feedback: 0.0,
            buf0: 0.0,
            buf1: 0.0,
            mode: FilterMode::Lowpass,
        };
        filter.calculate_feedback();

        filter
    }

    pub fn process(&mut self, input: f32) -> f32 {
        if input == 0.0 {
            return 0.0;
        }

        self.buf0 += self.cutoff * (input - self.buf0);
        self.buf1 += self.cutoff * (self.buf0 - self.buf1);

        match self.mode {
            FilterMode::Lowpass => self.buf1,
            FilterMode::Highpass => input - self.buf0,
            FilterMode::Bandpass => self.buf0 - self.buf1,
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.clamp(0.01, 0.99);
        self.calculate_feedback();
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.0, 1.0);
        self.calculate_feedback();
    }

    pub fn set_mode(&mut self, mode: FilterMode) {
        if self.mode != mode {
            self.mode = mode;
        }
    }

    fn calculate_feedback(&mut self) {
        self.feedback = self.resonance + self.resonance / (1.0 - self.cutoff);
    }
}

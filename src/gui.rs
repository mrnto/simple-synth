use std::sync::mpsc::Sender;
use self::control_mapping::normalize_to_time;
use crate::{
    commands::{SynthCommand, SynthParam},
    synthesizer::{EnvelopeStage, FilterMode, Waveform},
};

slint::include_modules!();

pub struct GuiController {
    sender: Sender<SynthCommand>,
}

impl GuiController {
    pub fn new(sender: Sender<SynthCommand>) -> Self {
        Self {
            sender,
        }
    }

    pub fn run_gui(&self) -> Result<(), slint::PlatformError> {
        let view = SynthWindow::new()?;

        self.connect_controls(&view);
        self.connect_keyboard(&view);

        view.run()?;

        Ok(())
    }

    fn connect_controls(&self, view: &SynthWindow) {
        let controls = view.global::<ControlsAdapter>();

        controls.on_attack_changed(self.handle_stage_change(EnvelopeStage::Attack, normalize_to_time));
        controls.on_decay_changed(self.handle_stage_change(EnvelopeStage::Decay, normalize_to_time));
        controls.on_sustain_changed(self.handle_stage_change(EnvelopeStage::Sustain, |x| x));
        controls.on_release_changed(self.handle_stage_change(EnvelopeStage::Release, normalize_to_time));

        controls.on_waveform_selected({
            let tx = self.sender.clone();
            move |waveform| {
                if let Ok(waveform) = waveform.parse::<Waveform>() {
                    if let Err(e) = tx.send(SynthCommand::SetParam(SynthParam::Waveform(waveform))) {
                        eprintln!("[gui] Failed to send SetWaveform message: {}", e);
                    }
                } else {
                    eprintln!("[gui] Invalid waveform selected: {}", waveform);
                }
            }
        });
        controls.on_filter_mode_selected({
            let tx = self.sender.clone();
            move |mode| {
                if let Ok(mode) = mode.parse::<FilterMode>() {
                    if let Err(e) = tx.send(SynthCommand::SetParam(SynthParam::FilterMode(mode))) {
                        eprintln!("[gui] Failed to send SetFilterMode message: {}", e);
                    }
                } else {
                    eprintln!("[gui] Invalid filter mode selected: {}", mode);
                }
            }
        });

        controls.on_cutoff_changed({
            let tx = self.sender.clone();
            move |value| {
                if let Err(e) = tx.send(SynthCommand::SetParam(SynthParam::Cutoff(value))) {
                    eprintln!("[gui] Failed to send SetCutoff message: {}", e);
                }
            }
        });
        controls.on_resonance_changed({
            let tx = self.sender.clone();
            move |value| {
                if let Err(e) = tx.send(SynthCommand::SetParam(SynthParam::Resonance(value))) {
                    eprintln!("[gui] Failed to send SetResonance message: {}", e);
                }
            }
        });
    }

    fn connect_keyboard(&self, view: &SynthWindow) {
        let keyboard = view.global::<KeyboardAdapter>();

        keyboard.on_key_pressed({
            let tx = self.sender.clone();
            move |note_number| {
                if let Err(e) = tx.send(SynthCommand::NoteOn(note_number as u8)) {
                    eprintln!("[gui] Failed to send NoteOn message: {}", e);
                }
            }
        });
        keyboard.on_key_released({
            let tx = self.sender.clone();
            move |note_number| {
                if let Err(e) = tx.send(SynthCommand::NoteOff(note_number as u8)) {
                    eprintln!("[gui] Failed to send NoteOff message: {}", e);
                }
            }
        });
    }

    fn handle_stage_change<F>(&self, stage: EnvelopeStage, f: F) -> impl Fn(f32) + 'static
    where
        F: Fn(f32) -> f32 + 'static,
    {
        let tx = self.sender.clone();
        move |value| {
            let time = f(value);
            if let Err(e) = tx.send(SynthCommand::SetParam(SynthParam::EnvelopeStage(stage, time))) {
                eprintln!("[gui] Failed to send SetStage ({:?}) message: {}", stage, e);
            }
        }
    }
}

mod control_mapping {
    pub fn normalize_to_time(y: f32) -> f32 {
        const BREAKPOINTS: &[(f32, f32)] = &[
            (0.01, 0.0), // 10ms -> 0
            (0.2, 0.2),  // 200ms -> 0.2
            (0.6, 0.4),  // 600ms -> 0.4
            (1.0, 0.6),  // 1 sec -> 0.6
            (5.0, 0.8),  // 5 sec -> 0.8
            (10.0, 1.0), // 10 sec -> 1
        ];

        BREAKPOINTS
            .windows(2)
            .find_map(|window| {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];

                if y >= y1 && y <= y2 {
                    Some(x1 + (y - y1) * (x2 - x1) / (y2 - y1))
                } else {
                    None
                }
            })
            .unwrap_or(10.0)
    }
}

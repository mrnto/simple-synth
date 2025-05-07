use std::sync::mpsc::Sender;
use crate::{
    messages::SynthMsg,
    synthesizer::{EnvelopeStage, Waveform},
};

slint::include_modules!();

pub struct GuiController {
    sender: Sender<SynthMsg>,
}

impl GuiController {
    pub fn new(sender: Sender<SynthMsg>) -> Self {
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
        
        controls.on_attack_changed({
            let tx = self.sender.clone();
            move |attack| {
                let attack_time = normalize_to_time(attack);
                let message = SynthMsg::SetStage(EnvelopeStage::Attack, attack_time);
                
                if let Err(e) = tx.send(message) {
                    eprintln!("Failed to send SetStage message: {}", e);
                }
            }
        });
        controls.on_decay_changed({
            let tx = self.sender.clone();
            move |decay| {
                let decay_time = normalize_to_time(decay);
                let message = SynthMsg::SetStage(EnvelopeStage::Decay, decay_time);
                
                if let Err(e) = tx.send(message) {
                    eprintln!("Failed to send SetStage message: {}", e);
                }
            }
        });
        controls.on_sustain_changed({
            let tx = self.sender.clone();
            move |sustain| {
                let message = SynthMsg::SetStage(EnvelopeStage::Sustain, sustain);
                
                if let Err(e) = tx.send(message) {
                    eprintln!("Failed to send SetStage message: {}", e);
                }
            }
        });
        controls.on_release_changed({
            let tx = self.sender.clone();
            move |release| {
                let release_time = normalize_to_time(release);
                let message = SynthMsg::SetStage(EnvelopeStage::Release, release_time);
                
                if let Err(e) = tx.send(message) {
                    eprintln!("Failed to send SetStage message: {}", e);
                }
            }
        });
        controls.on_waveform_selected({
            let tx = self.sender.clone();
            move |waveform| {
                if let Ok(waveform) = waveform.parse::<Waveform>() {
                    let message = SynthMsg::SetWaveform(waveform);
                    
                    if let Err(e) = tx.send(message) {
                        eprintln!("Failed to send SetWaveform message: {}", e);
                    }
                } else {
                    eprintln!("Invalid waveform selected: {}", waveform);
                }
            }
        });
    }
    
    fn connect_keyboard(&self, view: &SynthWindow) {
        let keyboard = view.global::<KeyboardAdapter>();
        
        keyboard.on_key_pressed({
            let tx = self.sender.clone();
            move |note_number| {
                let message = SynthMsg::NoteOn(note_number as u8);
                
                if let Err(e) = tx.send(message) {
                    eprintln!("Failed to send NoteOn message: {}", e);
                }
            }
        });
        keyboard.on_key_released({
            let tx = self.sender.clone();
            move |note_number| {
                let message = SynthMsg::NoteOff(note_number as u8);

                if let Err(e) = tx.send(message) {
                    eprintln!("Failed to send NoteOff message: {}", e);
                }
            }
        });
    }
}

fn normalize_to_time(y: f32) -> f32 {
    let breakpoints = vec![
        (0.01, 0.0), // 10ms -> 0
        (0.2, 0.2),  // 200ms -> 0.2
        (0.6, 0.4),  // 600ms -> 0.4
        (1.0, 0.6),  // 1 sec -> 0.6
        (5.0, 0.8),  // 5 sec -> 0.8
        (10.0, 1.0), // 10 sec -> 1
    ];

    for i in 0..breakpoints.len() - 1 {
        let (x1, y1) = breakpoints[i];
        let (x2, y2) = breakpoints[i + 1];

        if y >= y1 && y <= y2 {
            return x1 + (y - y1) * (x2 - x1) / (y2 - y1);
        }
    }

    10.0
}

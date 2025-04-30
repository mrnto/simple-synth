use crate::{
    audio_engine::AudioEngine,
    messages::{EnvelopeMsg, OscillatorMsg, SynthMsg},
    synthesizer::Waveform,
};
use std::sync::mpsc::Sender;

slint::include_modules!();

pub struct GuiController {
    sender: Sender<SynthMsg>,
}

impl GuiController {
    pub fn new(audio_engine: &AudioEngine) -> Self {
        Self {
            sender: audio_engine.clone_sender(),
        }
    }

    pub fn run_gui(&self) {
        let view = SynthWindow::new().expect("Failed to create view!");
        self.connect_controls(&view);
        self.connect_keyboard(&view);
        view.run().expect("Failed to run GUI!");
    }

    fn connect_controls(&self, view: &SynthWindow) {
        let controls = view.global::<ControlsAdapter>();
        controls.on_attack_changed({
            let tx = self.sender.clone();
            move |attack| {
                let attack_time = normalize_to_time(attack);
                tx.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetAttack(attack_time))).unwrap();
            }
        });
        controls.on_decay_changed({
            let tx = self.sender.clone();
            move |decay| {
                let decay_time = normalize_to_time(decay);
                tx.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetDecay(decay_time))).unwrap();
            }
        });
        controls.on_sustain_changed({
            let tx = self.sender.clone();
            move |sustain| {
                tx.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetSustain(sustain))).unwrap();
            }
        });
        controls.on_release_changed({
            let tx = self.sender.clone();
            move |release| {
                let release_time = normalize_to_time(release);
                tx.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetRelease(release_time))).unwrap();
            }
        });
        controls.on_waveform_selected({
            let tx = self.sender.clone();
            move |waveform| {
                if let Ok(waveform) = waveform.parse::<Waveform>() {
                    tx.send(SynthMsg::OscillatorMsg(OscillatorMsg::SetWaveform(waveform))).unwrap();
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
                tx.send(SynthMsg::OscillatorMsg(OscillatorMsg::NoteOn(note_number as u8))).unwrap();
            }
        });
        keyboard.on_key_released({
            let tx = self.sender.clone();
            move |note_number| {
                tx.send(SynthMsg::OscillatorMsg(OscillatorMsg::NoteOff(note_number as u8))).unwrap();
            }
        });
    }
}

fn normalize_to_time(y: f32) -> f32 {
    let breakpoints = vec![
        (0.01, 0.0),    // 10ms -> 0
        (0.2, 0.2),   // 200ms -> 0.2
        (0.6, 0.4),   // 600ms -> 0.4
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

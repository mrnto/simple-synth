// Hide console for Windows users
// https://docs.slint.dev/latest/docs/slint/guide/platforms/desktop/#handle-the-console-window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod audio_engine;
mod error;
mod synthesizer;
mod messages;
mod oscillator;
mod envelope;
mod voice;
mod voice_manager;
mod waveform;

use audio_engine::AudioEngine;
use gui::GuiController;
use waveform::Waveform;
use messages::{OscillatorMsg, SynthMsg};
use synthesizer::Synthesizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let synth = Synthesizer::new(44100);
    let audio = AudioEngine::new(synth)?;

    let tx_clone = audio.clone_sender();
    let rate = audio.sample_rate();

    audio.play()?;

    let gui = GuiController::new(&audio);
    gui.run_gui();

    Ok(())
}

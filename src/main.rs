// Hide console for Windows users
// https://docs.slint.dev/latest/docs/slint/guide/platforms/desktop/#handle-the-console-window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_engine;
mod gui;
mod messages;
mod error;
mod synthesizer;

use audio_engine::AudioEngine;
use gui::GuiController;
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

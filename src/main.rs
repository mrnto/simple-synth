// Hide console for Windows users (non-debug builds)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_engine;
mod gui;
mod messages;
mod error;
mod synthesizer;

use std::error::Error;
use audio_engine::AudioEngine;
use gui::GuiController;
use synthesizer::Synthesizer;

fn main() -> Result<(), Box<dyn Error>> {
    let synth = Synthesizer::new(44100);
    
    let audio = AudioEngine::new(synth)?;
    audio.play()?;

    let gui = GuiController::new(audio.clone_sender());
    gui.run_gui()?;

    Ok(())
}

// Hide console for Windows users (non-debug builds)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_engine;
mod gui;
mod commands;
mod error;
mod synthesizer;

use std::error::Error;
use audio_engine::AudioEngine;
use gui::GuiController;
use synthesizer::Synth;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = AudioEngine::new()?;
    let synth = Synth::new(audio.sample_rate());
    audio.add_synth(synth)?;
    audio.play()?;

    let gui = GuiController::new(audio.clone_sender()?);
    gui.run_gui()?;

    Ok(())
}

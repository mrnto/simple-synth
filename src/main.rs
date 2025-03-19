// Hide console for Windows users
// https://docs.slint.dev/latest/docs/slint/guide/platforms/desktop/#handle-the-console-window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod audio_engine;
mod error;
mod synthesizer;
mod synth_msg;
mod oscillator;
mod envelope;

use audio_engine::AudioEngine;
use gui::run;
use oscillator::Waveform;
use synth_msg::SynthMsg;
use synthesizer::Synthesizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let synth = Synthesizer::new();
    let audio = AudioEngine::new(synth)?;

    let tx_clone = audio.clone_sender();
    let rate = audio.sample_rate();
    let _ = tx_clone.send(SynthMsg::SetOscillator(Waveform::Sine, 0.0, rate));

    audio.play()?;

    run(&audio);

    Ok(())
}

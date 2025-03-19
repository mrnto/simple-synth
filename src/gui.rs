use crate::{audio_engine::AudioEngine, oscillator::Waveform, synth_msg::SynthMsg};

slint::include_modules!();

pub fn run(audio: &AudioEngine) {
    let synth_window = init(&audio);
    synth_window.run().expect("GUI run failed.");
}

fn init(audio: &AudioEngine) -> SynthWindow {
    let view_handle = SynthWindow::new().unwrap();

    connect_view_to_controller(&view_handle, &audio);

    view_handle
}

fn connect_view_to_controller(view_handle: &SynthWindow, audio: &AudioEngine) {
    view_handle.global::<ControlsAdapter>().on_select_waveform({
        let tx_clone = audio.clone_sender();
        move |waveform| {
            if let Some(waveform) = string_to_waveform(&waveform) {
                tx_clone.send(SynthMsg::SetWaveform(waveform)).unwrap();
            } else {
                eprintln!("Invalid waveform selected: {}", waveform);
            }
        }
    });
    view_handle.global::<KeyboardAdapter>().on_key_pressed({
        let tx_clone = audio.clone_sender();
        move |note, octave| {
            let frequency = note_to_frequency(&note, octave).unwrap();
            tx_clone.send(SynthMsg::SetFrequency(frequency)).unwrap();
            println!("{}{} {}", note, octave, frequency);
        }
    });
    view_handle.global::<KeyboardAdapter>().on_key_released({
        let tx_clone = audio.clone_sender();
        move |note, octave| {
            tx_clone.send(SynthMsg::SetFrequency(0.0)).unwrap();
        }
    });
}

fn string_to_waveform(waveform_str: &str) -> Option<Waveform> {
    match waveform_str {
        "Sine" => Some(Waveform::Sine),
        "Square" => Some(Waveform::Square),
        "Triangle" => Some(Waveform::Triangle),
        "Sawtooth" => Some(Waveform::Sawtooth),
        "Noise" => Some(Waveform::Noise),
        _ => None,
    }
}

fn note_to_frequency(note: &str, octave: i32) -> Result<f32, String> {
    let semitone_diff = note_to_semitone(note)?;
    let semitone_offset = (octave - 4) * 12 + semitone_diff;
    let frequency = 440.0 * 2.0_f32.powf(semitone_offset as f32 / 12.0);

    Ok(frequency)
}

fn note_to_semitone(note: &str) -> Result<i32, String> {
    match note {
        "C"  => Ok(-9),
        "C#" | "Db" => Ok(-8),
        "D"  => Ok(-7),
        "D#" | "Eb" => Ok(-6),
        "E"  => Ok(-5),
        "F"  => Ok(-4),
        "F#" | "Gb" => Ok(-3),
        "G"  => Ok(-2),
        "G#" | "Ab" => Ok(-1),
        "A"  => Ok(0),
        "A#" | "Bb" => Ok(1),
        "B"  => Ok(2),
        _ => Err(format!("Invalid note: {}", note)),
    }
}

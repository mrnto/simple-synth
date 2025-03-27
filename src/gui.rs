use crate::{audio_engine::AudioEngine, messages::{EnvelopeMsg, OscillatorMsg, SynthMsg}, oscillator::Waveform};

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
    // TODO: too many
    view_handle.global::<ControlsAdapter>().on_attack_changed({
        let tx_clone = audio.clone_sender();
        move |attack| {
            let attack_time = normalize_to_time(attack);
            println!("attack: {}", attack);
            println!("Normalized from 10ms to 10sec: {}", attack_time);
            tx_clone.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetAttack(attack_time/100.0))).unwrap();
        }
    });
    view_handle.global::<ControlsAdapter>().on_decay_changed({
        let tx_clone = audio.clone_sender();
        move |decay| {
            let decay_time = normalize_to_time(decay);
            tx_clone.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetDecay(decay/100.0))).unwrap();
        }
    });
    view_handle.global::<ControlsAdapter>().on_sustain_changed({
        let tx_clone = audio.clone_sender();
        move |sustain| {
            println!("Sustain level: {}", sustain);
            tx_clone.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetSustain(sustain))).unwrap();
        }
    });
    view_handle.global::<ControlsAdapter>().on_release_changed({
        let tx_clone = audio.clone_sender();
        move |release| {
            let release_time = normalize_to_time(release);
            tx_clone.send(SynthMsg::EnvelopeMsg(EnvelopeMsg::SetRelease(release/100.0))).unwrap();
        }
    });

    view_handle.global::<ControlsAdapter>().on_waveform_selected({
        let tx_clone = audio.clone_sender();
        move |waveform| {
            if let Some(waveform) = string_to_waveform(&waveform) {
                tx_clone.send(SynthMsg::OscillatorMsg(OscillatorMsg::SetWaveform(waveform))).unwrap();
            } else {
                eprintln!("Invalid waveform selected: {}", waveform);
            }
        }
    });
    view_handle.global::<KeyboardAdapter>().on_key_pressed({
        let tx_clone = audio.clone_sender();
        move |note, octave| {
            let frequency = note_to_frequency(&note, octave).unwrap();
            tx_clone.send(SynthMsg::OscillatorMsg(OscillatorMsg::SetFrequency(frequency))).unwrap();
            println!("{}{} {}", note, octave, frequency);
        }
    });
    view_handle.global::<KeyboardAdapter>().on_key_released({
        let tx_clone = audio.clone_sender();
        move |note, octave| {
            tx_clone.send(SynthMsg::OscillatorMsg(OscillatorMsg::SetFrequency(0.0))).unwrap();
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

fn normalize_to_time(y: f32) -> f32 {
    let breakpoints = vec![
        (10.0, 0.0),    // 10ms -> 0
        (200.0, 0.2),   // 200ms -> 0.2
        (600.0, 0.4),   // 600ms -> 0.4
        (1000.0, 0.6),  // 1 sec -> 0.6
        (5000.0, 0.8),  // 5 sec -> 0.8
        (10000.0, 1.0), // 10 sec -> 1
    ];

    for i in 0..breakpoints.len() - 1 {
        let (x1, y1) = breakpoints[i];
        let (x2, y2) = breakpoints[i + 1];

        if y >= y1 && y <= y2 {
            return x1 + (y - y1) * (x2 - x1) / (y2 - y1);
        }
    }

    10000.0
}

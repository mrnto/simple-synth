mod commands;
mod error;
mod synthesizer;

use std::sync::Arc;
use nih_plug::{prelude::*, util::db_to_gain};
use crate::synthesizer::VoiceManager;

struct SimpleSynth {
    params: Arc<SimpleSynthParams>,
    voice_manager: VoiceManager,
}

#[derive(Params)]
struct SimpleSynthParams {
    #[id = "gain"]
    gain: FloatParam,
}

impl Default for SimpleSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(SimpleSynthParams::default()),
            voice_manager: VoiceManager::new(44100),
        }
    }
}

impl Default for SimpleSynthParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear {
                    min: -10.0,
                    max: 10.0,
                },
            )
            .with_step_size(0.1)
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
        }
    }
}

impl Plugin for SimpleSynth {
    const NAME: &'static str = "SimpleSynth";
    const VENDOR: &'static str = "mrnto";
    const URL: &'static str = "https://github.com/mrnto/simple-synthesizer";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
         while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, .. } => {
                    self.voice_manager.note_on(note);
                }
                NoteEvent::NoteOff { note, .. } => {
                    self.voice_manager.note_off(note);
                }
                _ => {}
            }
        }

        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            let gain = db_to_gain(gain);
            for sample in channel_samples {
                let output = self.voice_manager.process_voices();
                *sample = output * util::db_to_gain_fast(gain);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SimpleSynth {
    const CLAP_ID: &'static str = "com.mrnto.simple-synth";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Simple synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Stereo,
        ClapFeature::Synthesizer,
    ];
}

impl Vst3Plugin for SimpleSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"mrntoSimpleSynth";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Stereo,
        Vst3SubCategory::Synth,
    ];
}

nih_export_clap!(SimpleSynth);
nih_export_vst3!(SimpleSynth);

use std::sync::Arc;
use nih_plug::{prelude::*, util::db_to_gain};
use nih_plug_iced::IcedState;

mod editor;

use simple_synth_core::{
    EnvelopeStage,
    FilterMode,
    Waveform,
    SynthParam,
    VoiceManager,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum WaveformParam {
    #[name = "Sine"]
    Sine,
    #[name = "Square"]
    Square,
    #[name = "Triangle"]
    Triangle,
    #[name = "Sawtooth"]
    Sawtooth,
    #[name = "Noise"]
    Noise,
}

impl From<WaveformParam> for Waveform {
    fn from(param: WaveformParam) -> Self {
        match param {
            WaveformParam::Sine => Waveform::Sine,
            WaveformParam::Square => Waveform::Square,
            WaveformParam::Triangle => Waveform::Triangle,
            WaveformParam::Sawtooth => Waveform::Sawtooth,
            WaveformParam::Noise => Waveform::Noise,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum FilterModeParam {
    #[name = "Lowpass"]
    Lowpass,
    #[name = "Highpass"]
    Highpass,
    #[name = "Bandpass"]
    Bandpass,
}

impl From<FilterModeParam> for FilterMode {
    fn from(param: FilterModeParam) -> Self {
        match param {
            FilterModeParam::Lowpass => FilterMode::Lowpass,
            FilterModeParam::Highpass => FilterMode::Highpass,
            FilterModeParam::Bandpass => FilterMode::Bandpass,
        }
    }
}

struct SimpleSynth {
    params: Arc<SimpleSynthParams>,
    voice_manager: VoiceManager,
}

#[derive(Params)]
struct SimpleSynthParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    #[id = "gain"]
    gain: FloatParam,
    #[id = "waveform"]
    waveform: EnumParam<WaveformParam>,
    #[id = "attack"]
    attack: FloatParam,
    #[id = "decay"]
    decay: FloatParam,
    #[id = "sustain"]
    sustain: FloatParam,
    #[id = "release"]
    release: FloatParam,
    #[id = "filter_mode"]
    filter_mode: EnumParam<FilterModeParam>,
    #[id = "cutoff"]
    cutoff: FloatParam,
    #[id = "resonance"]
    resonance: FloatParam,
    #[id = "filter_env_amount"]
    filter_env_amount: FloatParam,
    #[id = "filter_attack"]
    filter_attack: FloatParam,
    #[id = "filter_decay"]
    filter_decay: FloatParam,
    #[id = "filter_sustain"]
    filter_sustain: FloatParam,
    #[id = "filter_release"]
    filter_release: FloatParam,
}

impl Default for SimpleSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(SimpleSynthParams::default()),
            voice_manager: VoiceManager::new(44100.0),
        }
    }
}

impl Default for SimpleSynthParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear {
                    min: -20.0,
                    max: 20.0,
                },
            )
            .with_step_size(0.1)
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB"),
            waveform: EnumParam::new("Waveform", WaveformParam::Sine),
            attack: FloatParam::new(
                "Attack",
                10.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 5000.0
                })
                .with_unit(" ms"),
            decay: FloatParam::new(
                "Decay",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 5000.0
                })
                .with_unit(" ms"),
            sustain: FloatParam::new(
                "Sustain",
                0.8,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0
                }),
            release: FloatParam::new(
                "Release",
                300.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10000.0
                })
                .with_unit(" ms"),
            filter_mode: EnumParam::new("Filter mode", FilterModeParam::Lowpass),
            cutoff: FloatParam::new(
                "Cutoff",
                1.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                }
            ),
            resonance: FloatParam::new(
                "Resonance",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                }
            ),
            filter_env_amount: FloatParam::new(
                "Filter Env Amount",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                }
            ),
            filter_attack: FloatParam::new(
                "Attack",
                10.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 5000.0
                })
                .with_unit(" ms"),
            filter_decay: FloatParam::new(
                "Decay",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 5000.0
                })
                .with_unit(" ms"),
            filter_sustain: FloatParam::new(
                "Sustain",
                0.8,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0
                }),
            filter_release: FloatParam::new(
                "Release",
                300.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10000.0
                })
                .with_unit(" ms"),
        }
    }
}

impl Plugin for SimpleSynth {
    const NAME: &'static str = "SimpleSynth";
    const VENDOR: &'static str = "mrnto";
    const URL: &'static str = "https://github.com/mrnto/simple-synth";
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

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.voice_manager.apply_param(SynthParam::SampleRate(buffer_config.sample_rate));
        true
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
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

        self.voice_manager.apply_param(SynthParam::Waveform(self.params.waveform.value().into()));
        self.voice_manager.apply_param(SynthParam::EnvelopeStage(EnvelopeStage::Attack, self.params.attack.value() / 1000.0));
        self.voice_manager.apply_param(SynthParam::EnvelopeStage(EnvelopeStage::Decay, self.params.decay.value() / 1000.0));
        self.voice_manager.apply_param(SynthParam::EnvelopeStage(EnvelopeStage::Sustain, self.params.sustain.value()));
        self.voice_manager.apply_param(SynthParam::EnvelopeStage(EnvelopeStage::Release, self.params.release.value() / 1000.0));
        self.voice_manager.apply_param(SynthParam::FilterMode(self.params.filter_mode.value().into()));
        self.voice_manager.apply_param(SynthParam::Cutoff(self.params.cutoff.value()));
        self.voice_manager.apply_param(SynthParam::Resonance(self.params.release.value()));
        self.voice_manager.apply_param(SynthParam::FilterEnvAmount(self.params.filter_env_amount.value()));
        self.voice_manager.apply_param(SynthParam::FilterEnvStage(EnvelopeStage::Attack, self.params.filter_attack.value() / 1000.0));
        self.voice_manager.apply_param(SynthParam::FilterEnvStage(EnvelopeStage::Decay, self.params.filter_decay.value() / 1000.0));
        self.voice_manager.apply_param(SynthParam::FilterEnvStage(EnvelopeStage::Sustain, self.params.filter_sustain.value()));
        self.voice_manager.apply_param(SynthParam::FilterEnvStage(EnvelopeStage::Release, self.params.filter_release.value() / 1000.0));

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

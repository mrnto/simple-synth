use std::sync::mpsc::{self, Receiver, Sender};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError,
    SampleFormat, SizedSample, FromSample,
    Host, Device, SupportedStreamConfig, Stream, OutputCallbackInfo,
    default_host,
};
use crate::{
    error::AudioEngineError,
    commands::SynthCommand,
    synthesizer::Synth,
};

// TODO: don't like it, needs a lot of improvements and changes
#[allow(dead_code)]
pub struct AudioEngine {
    host: Host,
    device: Device,
    config: SupportedStreamConfig,
    stream: Option<Stream>,
    sender: Option<Sender<SynthCommand>>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, AudioEngineError> {
        let (host, device, config) = Self::device_setup()?;

        Ok(Self {
            host,
            device,
            config,
            stream: None,
            sender: None,
        })
    }

    pub fn add_synth(&mut self, synth: Synth) -> Result<(), AudioEngineError> {
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        self.stream_setup(synth, rx)?;

        Ok(())
    }

    pub fn clone_sender(&self) -> Result<Sender<SynthCommand>, AudioEngineError> {
        self.sender.clone().ok_or(AudioEngineError::SenderUnavailable)
    }

    pub fn play(&self) -> Result<(), AudioEngineError> {
        if let Some(stream) = &self.stream {
            stream.play()?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn pause(&self) -> Result<(), AudioEngineError> {
        if let Some(stream) = &self.stream {
            stream.pause()?
        }

        Ok(())
    }

    pub fn sample_rate(&self) -> u32 {
        self.config.config().sample_rate.0
    }

    fn device_setup() -> Result<(Host, Device, SupportedStreamConfig), AudioEngineError> {
        let host = default_host();
        let device = host.default_output_device().ok_or(AudioEngineError::NoDefaultDevice)?;
        let config = device.default_output_config()?;

        println!("[audio] Output device: {}", device.name()?);
        println!("[audio] Default output config: {:?}", config);

        Ok((host, device, config))
    }

    fn stream_setup(&mut self, synth: Synth, rx: Receiver<SynthCommand>) -> Result<(), AudioEngineError> {
        let stream = match self.config.sample_format() {
            SampleFormat::F32 => self.make_stream::<f32>(synth, rx)?,
            SampleFormat::I16 => self.make_stream::<i16>(synth, rx)?,
            SampleFormat::U16 => self.make_stream::<u16>(synth, rx)?,
            _ => return Err(AudioEngineError::BuildStreamError(BuildStreamError::StreamConfigNotSupported)),
        };
        self.stream = Some(stream);

        Ok(())
    }

    fn make_stream<T>(&mut self, mut synth: Synth, rx: Receiver<SynthCommand>) -> Result<Stream, AudioEngineError>
    where 
        T: SizedSample + FromSample<f32>,
    {
        let config = self.config.config();
        let channels = config.channels as usize;
        let err_fn = |e| eprintln!("[audio] Error building output sound stream: {}", e);

        let stream = self.device.build_output_stream(
            &config,
            move |output: &mut [T], _: &OutputCallbackInfo| {
                while let Ok(command) = rx.try_recv() {
                    synth.handle_command(command);
                }

                // self.process_frame(output, channels)
                for frame in output.chunks_mut(channels) {
                    let value = T::from_sample(synth.generate());

                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }

    // fn process_frame<SampleType>(&self, output: &mut [SampleType], synthesizer: &mut Synthesizer, channels: usize)
    // where
    //     SampleType: Sample + FromSample<f32>,
    // {
    //     for frame in output.chunks_mut(channels) {
    //         let value = SampleType::from_sample(synthesizer.generate());

    //         for sample in frame.iter_mut() {
    //             *sample = value;
    //         }
    //     }
    // }
}

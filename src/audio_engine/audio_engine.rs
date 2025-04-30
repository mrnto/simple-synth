use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, PlayStreamError, PauseStreamError,
    SampleFormat, SizedSample, FromSample,
    Host, Device, SupportedStreamConfig, Stream, OutputCallbackInfo,
    default_host,
};
use std::sync::mpsc::{self, Receiver, Sender};
use crate::{
    // error::DeviceSetupError,
    audio_engine::DeviceSetupError,
    messages::SynthMsg,
    synthesizer::Synthesizer,
};

pub struct AudioEngine {
    host: Host,
    device: Device,
    config: SupportedStreamConfig,
    stream: Option<Stream>,
    sender: Sender<SynthMsg>,
}

impl AudioEngine {
    pub fn new(synth: Synthesizer) -> Result<Self, Box<dyn std::error::Error>> {
        let (host, device, config) = Self::device_setup()?;
        
        let (tx, rx) = mpsc::channel();

        let mut engine = Self {
            host,
            device,
            config,
            stream: None,
            sender: tx,
        };

        engine.stream_setup(synth, rx)?;

        Ok(engine)
    }

    pub fn clone_sender(&self) -> Sender<SynthMsg> {
        self.sender.clone()
    }

    pub fn play(&self) -> Result<(), PlayStreamError> {
        if let Some(stream) = &self.stream {
            stream.play()?;
        }

        Ok(())
    }

    pub fn pause(&self) -> Result<(), PauseStreamError> {
        if let Some(stream) = &self.stream {
            stream.pause()?
        }

        Ok(())
    }

    pub fn sample_rate(&self) -> u32 {
        self.config.config().sample_rate.0
    }

    fn device_setup() -> Result<(Host, Device, SupportedStreamConfig), DeviceSetupError> {
        let host = default_host();
    
        let device = host
            .default_output_device()
            .ok_or(DeviceSetupError::NoDefaultDevice)?;
        
        let config = device.default_output_config()?;
        
        println!("Output device: {}", device.name()?);
        println!("Default output config: {:?}", config);
    
        Ok((host, device, config))
    }

    fn stream_setup(&mut self, synthesizer: Synthesizer, rx: Receiver<SynthMsg>) -> Result<(), BuildStreamError> {
        let stream = match self.config.sample_format() {
            SampleFormat::F32 => self.make_stream::<f32>(synthesizer, rx)?,
            SampleFormat::I16 => self.make_stream::<i16>(synthesizer, rx)?,
            SampleFormat::U16 => self.make_stream::<u16>(synthesizer, rx)?,
            _ => return Err(BuildStreamError::StreamConfigNotSupported),
        };

        self.stream = Some(stream);
        Ok(())
    }

    fn make_stream<T>(&mut self, mut synthesizer: Synthesizer, rx: Receiver<SynthMsg>) -> Result<Stream, BuildStreamError>
    where 
        T: SizedSample + FromSample<f32>,
    {
        let config = self.config.config();
        let channels = config.channels as usize;
        let _sample_rate = config.sample_rate.0;

        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

        let stream = self.device.build_output_stream(
            &config,
            move |output: &mut [T], _: &OutputCallbackInfo| {
                while let Ok(message) = rx.try_recv() {
                    synthesizer.handle_message(message);
                }
                
                // self.process_frame(output, channels)
                for frame in output.chunks_mut(channels) {
                    let value = T::from_sample(synthesizer.generate());
        
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

    // fn process_frame<SampleType>(&mut self, output: &mut [SampleType], channels: usize)
    // where
    //     SampleType: Sample + FromSample<f32>,
    // {
    //     for frame in output.chunks_mut(channels) {
    //         let value = SampleType::from_sample(self.synthesizer.next_sample());

    //         for sample in frame.iter_mut() {
    //             *sample = value;
    //         }
    //     }
    // }
}

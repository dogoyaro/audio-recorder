use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{StreamConfig,Device,Stream};

pub struct AudioDevice {
    pub device: Device,
    pub config: StreamConfig
}

impl AudioDevice {
    fn new(device: Device, config: StreamConfig) -> Self {
        Self {device, config}
    }

    pub fn create_input_stream<F>(&self, data_callback: F) -> Result<Stream, cpal::BuildStreamError> 
    where
        F: FnMut(&[f32], &cpal::InputCallbackInfo) + Send + 'static,
    {
        self.device.build_input_stream(
            &self.config,
            data_callback,
            move |_err| {
    
            },
            None
        )
    }
}

pub fn record() {
    let audio_device = get_default_device();
    let data_callback = move |data: &[f32], _:&cpal::InputCallbackInfo| {
        println!("{:?}", data);
    };

    let stream = audio_device.create_input_stream(data_callback).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

}

pub fn get_default_device() -> AudioDevice {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device available");

    let mut supported_configs_range = input_device
        .supported_input_configs().expect("error while querying configs");

    let supported_config = supported_configs_range.next()
        .expect("no supported config")
        .with_max_sample_rate()
        .config();

    AudioDevice::new(input_device, supported_config)
}
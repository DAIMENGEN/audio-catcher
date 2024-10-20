use cpal::traits::HostTrait;

pub struct AudioDevice {
    pub index: usize,
    pub device: cpal::Device,
}

pub fn get_input_devices() -> Vec<AudioDevice> {
    let mut my_vec: Vec<AudioDevice> = Vec::new();
    let host = cpal::default_host();
    let input_devices = host.input_devices().unwrap();
    for (index, device) in input_devices.enumerate() {
        let audio_device = AudioDevice { index, device };
        my_vec.push(audio_device);
    }
    my_vec
}

pub fn get_output_devices() -> Vec<AudioDevice> {
    let mut my_vec: Vec<AudioDevice> = Vec::new();
    let host = cpal::default_host();
    let output_devices = host.output_devices().unwrap();
    for (index, device) in output_devices.enumerate() {
        let audio_device = AudioDevice { index, device };
        my_vec.push(audio_device);
    }
    my_vec
}
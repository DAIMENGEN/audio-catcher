use crate::utils::audio_wav_writer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavWriter;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Index;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub enum AudioDeviceType {
    Input, // 输入设备
    Output, // 输出设备
}

pub struct AudioDevice {
    pub index: usize,
    pub device: cpal::Device,
}

pub fn build_stream(
    device: &cpal::Device,
    config: cpal::SupportedStreamConfig,
    writer: Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>,
    err_fn: impl Fn(cpal::StreamError) + Send + 'static,
) -> cpal::Stream {
    match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<i8, i8>(data, &writer),
            err_fn,
            None,
        ).unwrap(),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<i16, i16>(data, &writer),
            err_fn,
            None,
        ).unwrap(),
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<i32, i32>(data, &writer),
            err_fn,
            None,
        ).unwrap(),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<f32, f32>(data, &writer),
            err_fn,
            None,
        ).unwrap(),
        _ => {
            panic!("Unsupported sample format: {:?}", config.sample_format())
        }
    }
}

pub fn catcher_device_audio(device_type: AudioDeviceType, device_index: usize, path: PathBuf) {
    let devices = match device_type {
        AudioDeviceType::Input => get_input_devices(),
        AudioDeviceType::Output => get_output_devices(),
    };
    let device = &devices.index(device_index).device;
    println!("Selected device: {}", device.name().unwrap());
    let config = match device_type {
        AudioDeviceType::Input => device.default_input_config(),
        AudioDeviceType::Output => device.default_output_config(),
    }.expect("Failed to get default config");
    println!("Device config: {:?}", config);
    let spec = audio_wav_writer::wav_spec_from_config(&config);
    let writer = audio_wav_writer::create_wav_writer_with_thread_safe(&path, spec);
    let writer_clone = writer.clone();
    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);
    let stream = build_stream(device, config, writer_clone, err_fn);
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize().unwrap();
    println!("Recording {} complete!", path.to_str().unwrap());
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


use audio_catcher::utils::audio_device;
use audio_catcher::utils::audio_device::AudioDevice;
use audio_catcher::utils::audio_wav_writer;
use cpal::traits::{DeviceTrait, StreamTrait};
use std::env;

fn main() {
    // Set up the input device and stream with the default input config.
    let input_devices: Vec<AudioDevice> = audio_device::get_input_devices();
    for device in input_devices.iter() {
        println!("{}:{}", device.index, device.device.name().unwrap());
    }
    let w820nb = &input_devices.first().unwrap().device;

    println!("Input device: {}", w820nb.name().unwrap());

    let config = w820nb
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    let mut path = env::current_dir().unwrap();
    path.push("recorded.wav");
    // const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
    let spec = audio_wav_writer::wav_spec_from_config(&config);
    let writer = audio_wav_writer::create_wav_writer_with_thread_safe(&path, spec);

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => w820nb.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        ).unwrap(),
        cpal::SampleFormat::I16 => w820nb.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        ).unwrap(),
        cpal::SampleFormat::I32 => w820nb.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        ).unwrap(),
        cpal::SampleFormat::F32 => w820nb.build_input_stream(
            &config.into(),
            move |data, _: &_| audio_wav_writer::write_data::<f32, f32>(data, &writer_2),
            err_fn,
            None,
        ).unwrap(),
        _ => {
            panic!("Unsupported sample format: {:?}", config.sample_format())
        }
    };

    stream.play().unwrap();

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize().unwrap();
    println!("Recording {} complete!", path.to_str().unwrap());
}


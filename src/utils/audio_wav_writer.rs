use cpal::FromSample;
use hound::WavWriter;
use std::fs::File;
use std::io::BufWriter;
use std::path;
use std::sync::{Arc, Mutex};

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

pub fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        // 采样率
        sample_rate: config.sample_rate().0 as _,
        // 样本格式
        sample_format: sample_format(config.sample_format()),
        // 样本大小
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
    }
}

pub fn create_wav_writer<P: AsRef<path::Path>>(filename: P, spec: hound::WavSpec) -> WavWriter<BufWriter<File>> {
    hound::WavWriter::create(filename, spec).unwrap()
}

pub fn create_wav_writer_with_thread_safe<P: AsRef<path::Path>>(filename: P, spec: hound::WavSpec) -> Arc<Mutex<Option<WavWriter<BufWriter<File>>>>> {
    let writer = create_wav_writer(filename, spec);
    Arc::new(Mutex::new(Some(writer)))
}

pub fn write_data<T, U>(data: &[T], writer: &WavWriterHandle)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in data.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
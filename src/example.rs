use crate::utils::audio_device;
use crate::utils::audio_device::AudioDeviceType;
use std::env;

pub fn catcher_microphone() {
    let path = env::current_dir().unwrap().join("recorded_microphone.wav");
    audio_device::catcher_device_audio(AudioDeviceType::Input, 1, path);
}

pub fn catcher_speaker() {
    let path = env::current_dir().unwrap().join("recorded_speaker.wav");
    audio_device::catcher_device_audio(AudioDeviceType::Output, 0, path);
}
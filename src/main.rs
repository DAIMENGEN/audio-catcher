use audio_catcher::example;

fn main() {
    // example::catcher_speaker();
    // example::catcher_microphone();
    let catcher_speaker_thread = std::thread::spawn(example::catcher_speaker);
    let catcher_microphone_thread = std::thread::spawn(example::catcher_microphone);
    catcher_speaker_thread.join().unwrap();
    catcher_microphone_thread.join().unwrap();

    // for audio_device in audio_device::get_input_devices() {
    //     println!("{}", audio_device.device.name().unwrap())
    // }
    //
    // for audio_device in audio_device::get_output_devices() {
    //     println!("{}", audio_device.device.name().unwrap())
    // }
}


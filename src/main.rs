use enigo::{Enigo, Settings};
use std::{
    sync::{Arc, Mutex},
    time::Duration
};
use vosk::{Recognizer,Model};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleFormat};

pub mod input_processing;
pub mod global_state;

use crate::global_state::global_state::{GLOBAL_STATE, init_global_state};
use crate::input_processing::input_processing::process_input;

fn main() {
    //Initialize Global Variables
    init_global_state(); 

    //Set up input simulator
    let mut _enigo = Enigo::new(&Settings::default()).unwrap();

    //Set up Mic Input
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("Missing Input Device");

    let mut supported_configs = input_device.supported_input_configs().expect("Error Querying Device Configs");
    let supported_config = supported_configs.next().expect("No Supported Configs!").with_max_sample_rate();

    let sample_rate = supported_config.sample_rate().0;
    let channels = supported_config.channels();
    let sample_format = supported_config.sample_format();

    let config = supported_config.into();
    let err_fn = |err| eprint!("Error Occured on the Input Audio Stream: {}", err);


    //Set up Vosk
    let model = Model::new("model/").unwrap();
    let mut recognizer = Recognizer::new(&model, sample_rate as f32).expect("Couldn't create Recognizer");

    recognizer.set_max_alternatives(1);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    let recognizer = Arc::new(Mutex::new(recognizer));
    let recognizer_clone = recognizer.clone();

    //Create input stream 
    let stream = match sample_format{
       SampleFormat::F32 => input_device.build_input_stream(&config, move |data: &[f32], _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels),err_fn , None),
       SampleFormat::I16 => input_device.build_input_stream(&config, move |data: &[i16], _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels),err_fn , None),
       SampleFormat::U16 => input_device.build_input_stream(&config, move |data: &[u16], _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels),err_fn , None),
       SampleFormat::U8  => input_device.build_input_stream(&config, move |data: &[u8],  _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels),err_fn , None),
       sample_format => panic!("Unsupported Sample Format '{sample_format}'")
    }.unwrap();


    stream.play().unwrap();
    while GLOBAL_STATE.get().read().unwrap().run == true {
        std::thread::sleep(Duration::from_millis(500));
    }     
}

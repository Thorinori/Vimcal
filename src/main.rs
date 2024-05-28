use enigo::{self, Enigo, Settings};
use std::{
    sync::{Arc, Mutex},
    time::Duration
};
use dasp::{sample::Sample, sample::ToSample};
use vosk::{self, Recognizer};
use cpal::{self, traits::{DeviceTrait, HostTrait, StreamTrait}, ChannelCount, SampleFormat};


fn main() {
    //Set up input simulator
    let mut _enigo = Enigo::new(&Settings::default()).unwrap();

    //Set up Mic Input
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("Missing Input Device");
    println!("Input Device: {}", input_device.name().unwrap());
    let mut supported_configs = input_device.supported_input_configs().expect("Error Querying Device Configs");
    let supported_config = supported_configs.next().expect("No Supported Configs!").with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0;
    let channels = supported_config.channels();
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let err_fn = |err| eprint!("Error Occured on the Input Audio Stream: {}", err);


    //Set up Vosk
    let mut model = vosk::Model::new("model/").unwrap();
    let mut recognizer = vosk::Recognizer::new(&model, sample_rate as f32).expect("Couldn't create Recognizer");

    recognizer.set_max_alternatives(1);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    let recognizer = Arc::new(Mutex::new(recognizer));

    let recognizer_clone = recognizer.clone();

    let words = ["space", "control", "alt", "tab", "enter", "back", 
                "slash", "comma", "apostrophe", "tick","period","mod",
                "dot" ,"up", "down", "left", "right", "curly", "brace",
                "left", "right", "parenthesis", "parentheses", "plus", "minus",
                "ampersand", "pound", "exclamation", "point", "question",
                "mark", "semi", "colon", "quotation", "at", "percent", "carrot",
                "star", "splat", "under", "score", "underscore", "bracket", "angle",
                "pipe", "escape", "function", "a", "b", "c", "d", "e", "f", "g", "h",
                "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                "w", "x", "y", "z", "zed", "one", "two", "three", "four", "five", "fix", "seven",
                "eight", "nine", "ten", "zero", "negative", "positive", "hundred", "thousand",
                "million", "billion", "trillion", "page", "home", "insert", "delete", "end",
                "scroll", "lock", "print", "screen", "pause", "num", "dollar", "shift", "return",
                "escape", "quit", "exit", "begin", "stop", "input"
    ];
    for word in words{
        if let None = model.find_word(word) {
            println!("Model couldn't find word: {}", word);
        }
    }
 
   //Create input stream 
    let stream = match sample_format{
        SampleFormat::F32 => input_device.build_input_stream(&config, move |data: &[f32], _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels),err_fn , None),
        SampleFormat::I16 => input_device.build_input_stream(&config, move |data: &[i16], _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels),err_fn , None),
        SampleFormat::U16 => input_device.build_input_stream(&config, move |data: &[u16], _| process_input(&mut recognizer.lock().unwrap(), data, channels),err_fn , None),
        SampleFormat::U8  => input_device.build_input_stream(&config, move |data: &[u8], _| process_input(&mut recognizer_clone.lock().unwrap(), data, channels), err_fn , None),
        sample_format => panic!("Unsupported Sample Format '{sample_format}'")
    }.unwrap();
   
   

    stream.play().unwrap();
    // std::thread::sleep(Duration::from_secs(2));
}

fn process_input<T: Sample + ToSample<i16>>(recognizer: &mut Recognizer, data: &[T], channels: ChannelCount){
    let data: Vec<i16> = data.iter().map(|v| v.to_sample()).collect();
    let data = if channels != 1{
        stereo_to_mono(&data)
    } else{
        data
    };

    let state = recognizer.accept_waveform(&data);
    match state {
        vosk::DecodingState::Running => {}
        vosk::DecodingState::Finalized => {
            print!("Word Recognized: {:#?}", recognizer.result().multiple().unwrap())
        }
        vosk::DecodingState::Failed => eprintln!("Recognizer Errored"),
    }
}

fn stereo_to_mono(input_data: &[i16]) -> Vec<i16> {
    let mut res = Vec::with_capacity(input_data.len()/2);
    res.extend(input_data.chunks_exact(2).map(|chunk| chunk[0]/2 + chunk[1]/2));
    res
}

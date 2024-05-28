use dasp::{sample::Sample, sample::ToSample};
use cpal::ChannelCount;
use vosk::Recognizer;
use crate::global_state;

pub fn process_input<T: Sample + ToSample<i16>>(recognizer: &mut Recognizer, data: &[T], channels: ChannelCount){
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
            let res = recognizer.result().multiple().unwrap();
            let text = res.alternatives[0].text;
            if global_state::GLOBAL_STATE.get().read().unwrap().listen{
                if text.eq("quit") || text.eq("exit"){
                    println!("Exiting Vimcal...");   
                    global_state::GLOBAL_STATE.get().write().unwrap().run = false;
                }else if text.eq("disable listening"){
                    global_state::GLOBAL_STATE.get().write().unwrap().listen = false;
                    println!("Disabling listening...");
                }else{
                    if !text.eq(""){
                        println!("Word was: {}", text);
                    }
                }
            } else if text.eq("enable listening") {
                global_state::GLOBAL_STATE.get().write().unwrap().listen = true; 
                println!("Enabling listening...");
            }
        }
        vosk::DecodingState::Failed => eprintln!("Recognizer Errored"),
    }
}

fn stereo_to_mono(input_data: &[i16]) -> Vec<i16> {
    let mut res = Vec::with_capacity(input_data.len()/2);
    res.extend(input_data.chunks_exact(2).map(|chunk| chunk[0]/2 + chunk[1]/2));
    res
}

mod oscilator;
mod wavetables;
use std::time::Duration;
use rodio::{OutputStream, Source};



fn main() {
    let wavetable: Vec<f32> = wavetables::mk_sine(64);
    
    let mut osc1 = oscilator::Oscillator::new(44100, wavetable);
    osc1.set_freq(440.0);
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(osc1.convert_samples());

    std::thread::sleep(Duration::from_secs(1));

}

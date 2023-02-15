pub fn mk_sine(wavetable_size: usize) -> Vec<f32>{
    let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);

    for i in 0..wavetable_size {
        wavetable.push((2.0 * std::f32::consts::PI * i as f32 / wavetable_size as f32).sin())
    }

    return wavetable
}

    

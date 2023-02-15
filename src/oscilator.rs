use std::time::Duration;
use rodio::Source;

pub struct Oscillator {
    sample_rate: u32,
    wavetable: Vec<f32>,
    index: f32,
    increment: f32,
}

impl Oscillator {
    pub fn new(sample_rate: u32, wavetable: Vec<f32>) -> Oscillator {
        return Oscillator {
            sample_rate: sample_rate,
            wavetable: wavetable,
            index: 0.0,
            increment: 0.0,
        }
    }

    pub fn set_freq(&mut self, freq: f32){
        self.increment = freq * self.wavetable.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.increment;
        self.index %= self.wavetable.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32{
        let trunc_index = self.index as usize;
        let next_index = (trunc_index + 1) % self.wavetable.len();

        let next_index_weight = self.index - trunc_index as f32;
        let trunc_index_weight = 1.0 - next_index_weight;

        return trunc_index_weight * self.wavetable[trunc_index] + next_index_weight * self.wavetable[next_index];

    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());

    }
}

impl Source for Oscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}



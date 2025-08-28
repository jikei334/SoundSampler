use std::f32::consts::PI;

use crate::note::Note;
use crate::source::SoundSource;


const DEFAULT_SAMPLE_RATE: u32 = 48_000;
const DEFAULT_DURATION_SECONDS: f32 = 2.0f32;
const FREQUENCY_C4: f32 = 261.6256f32;

pub struct Triangle;

impl SoundSource for Triangle {
    fn get_base(&self) -> Note {
        let length = (DEFAULT_DURATION_SECONDS * DEFAULT_SAMPLE_RATE as f32) as usize;
        let data = (0..length)
            .map(|i| {
                let t = i as f32 / DEFAULT_SAMPLE_RATE as f32;
                (2.0 / PI) * ((2.0 * PI * FREQUENCY_C4 * t).sin()).asin()
            })
        .collect();

        Note::new(data, DEFAULT_SAMPLE_RATE)
    }
}

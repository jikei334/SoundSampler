use std::path::PathBuf;

use hound::{WavReader, SampleFormat};

use crate::note::Note;
use crate::source::SoundSource;


pub struct Sampler {
    note: Note,
}

impl Sampler {
    pub fn new(src_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = WavReader::open(src_file)?;
        let spec = reader.spec();

        let data: Vec<f32> = if spec.sample_format == SampleFormat::Float {
            reader
                .samples::<f32>()
                .map(|s| s.unwrap_or(0f32))
                .collect()
        } else {
            reader
                .samples::<i32>()
                .map(|s| s.unwrap_or(0) as f32 / i16::MAX as f32)
                .collect()
        };

        Ok(Self {
            note: Note::new(data, spec.sample_rate),
        })
    }
}

impl SoundSource for Sampler {
    fn get_base(&self) -> Note {
        self.note.clone()
    }
}

use std::path::PathBuf;

use hound::{WavReader, SampleFormat};

use crate::note::Note;
use crate::score::ScorePartSource;
use crate::source::SoundSource;


pub struct Sampler {
    src_file: PathBuf,
    note: Note,
}

impl Sampler {
    pub fn new(src_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = WavReader::open(src_file.to_owned())?;
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
            src_file,
            note: Note::new(data, spec.sample_rate, None),
        })
    }

    pub fn src_file(&self) -> &PathBuf {
        &self.src_file
    }
}

impl SoundSource for Sampler {
    fn get_base(&self) -> Note {
        self.note.clone()
    }

    fn to_score_part_source(&self) -> ScorePartSource {
        ScorePartSource::Sampler(self.src_file.to_path_buf())
    }
}

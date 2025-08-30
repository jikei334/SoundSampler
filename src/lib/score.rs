use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::source::SoundSource;
use crate::source::sampler::Sampler;
use crate::source::sin::Sin;
use crate::source::triangle::Triangle;
use crate::track::{InstrumentTrack, Mixdown};

const DEFAULT_VOLUME: f32 = 1.0f32;


#[derive(Deserialize, Serialize)]
enum ScorePartSource {
    Sampler(PathBuf),
    Sin,
    Triangle,
}

impl From<ScorePartSource> for Result<Box<dyn SoundSource>, Box<dyn Error>> {
    fn from(score_part_source: ScorePartSource) -> Self {
        match score_part_source {
            ScorePartSource::Sampler(src_file) => {
                let sound_source = Sampler::new(src_file)?;
                Ok(Box::new(sound_source))
            },
            ScorePartSource::Sin => {
                Ok(Box::new(Sin))
            },
            ScorePartSource::Triangle => {
                Ok(Box::new(Triangle))
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ScoreNote {
    semitone: Option<f32>,
    start: Option<f32>,
    length: f32,
}

#[derive(Deserialize, Serialize)]
struct ScorePart {
    source: ScorePartSource,
    bpm: f32,
    score_notes: Vec<ScoreNote>,
    sample_rate: Option<u32>,
    volume: Option<f32>,
    channel: Option<u16>,
}

impl ScorePart {
    const DEFAULT_CHANNEL: u16 = 0u16;

    pub fn channel(&self) -> u16 {
        match self.channel {
            Some(channel) => channel,
            None => Self::DEFAULT_CHANNEL,
        }
    }
}

impl From<ScorePart> for Result<InstrumentTrack, Box<dyn Error>> {
    fn from(score_part: ScorePart) -> Self {
        let source: Result<Box<dyn SoundSource>, Box<dyn Error>> = score_part.source.into();
        let source = source?;
        let sample_rate = match score_part.sample_rate {
            Some(sample_rate) => sample_rate,
            None => source.sample_rate(),
        };
        let volume = match score_part.volume {
            Some(volume) => volume,
            None => DEFAULT_VOLUME,
        };

        let mut track = InstrumentTrack::new(sample_rate, volume);

        for score_note in score_part.score_notes {
            let start = match score_note.start {
                Some(start) => Some(60f32 / score_part.bpm * start),
                None => None,
            };
            let length = 60f32 / score_part.bpm * score_note.length;
            track.add_note(start, source.get_note(length, score_note.semitone));
        }

        Ok(track)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Score {
    num_channel: u16,
    sample_rate: u32,
    tracks: Vec<ScorePart>,
}

impl From<Score> for Result<Mixdown, Box<dyn Error>> {
    fn from(score: Score) -> Self {
        let mut mixdown = Mixdown::new(score.num_channel, score.sample_rate);
        for track in score.tracks {
            let channel = track.channel();
            let instrument_track: Result<InstrumentTrack, Box<dyn Error>> = track.into();
            mixdown.add_track(channel, instrument_track?)?;
        }

        Ok(mixdown)
    }
}

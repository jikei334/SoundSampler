use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::source::SoundSource;
use crate::source::sampler::Sampler;
use crate::source::sin::Sin;
use crate::source::triangle::Triangle;
use crate::track::{InstrumentTrack, Mixdown};

const DEFAULT_VOLUME: f32 = 1.0f32;


#[derive(Clone, Deserialize, Serialize)]
pub enum ScorePartSource {
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
pub struct ScoreNote {
    semitone: Option<f32>,
    start: Option<f32>,
    length: f32,
}

impl ScoreNote {
    pub fn new(semitone: Option<f32>, start: Option<f32>, length: f32) -> Self {
        Self {
            semitone,
            start,
            length,
        }
    }

    pub fn semitone(&self) -> Option<f32> {
        self.semitone
    }

    pub fn start(&self) -> Option<f32> {
        self.start
    }

    pub fn length(&self) -> f32 {
        self.length
    }
}

#[derive(Deserialize, Serialize)]
pub struct ScorePart {
    source: ScorePartSource,
    bpm: f32,
    score_notes: Vec<ScoreNote>,
    volume: Option<f32>,
    channel: Option<u16>,
}

impl ScorePart {
    const DEFAULT_CHANNEL: u16 = 0u16;

    pub fn new(source: ScorePartSource, bpm: f32, score_notes: Vec<ScoreNote>,
        volume: Option<f32>, channel: Option<u16>) -> Self {
        Self {
            source,
            bpm,
            score_notes,
            volume,
            channel,
        }
    }

    pub fn source(&self) -> ScorePartSource {
        self.source.clone()
    }

    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    pub fn score_notes(&self) -> &Vec<ScoreNote> {
        &self.score_notes
    }

    pub fn volume(&self) -> Option<f32> {
        self.volume
    }

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
        let volume = match score_part.volume {
            Some(volume) => volume,
            None => DEFAULT_VOLUME,
        };

        let mut track = InstrumentTrack::new(source.sample_rate(), volume);

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

impl Score {
    pub fn new(num_channel: u16, sample_rate: u32, tracks: Vec<ScorePart>) -> Self {
        Self {
            num_channel,
            sample_rate,
            tracks,
        }
    }

    pub fn num_channel(&self) -> u16 {
        self.num_channel
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn tracks(&self) -> &Vec<ScorePart> {
        &self.tracks
    }
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

use std::error::Error;
use std::path::PathBuf;

use hound::{WavWriter, WavSpec, SampleFormat};

use crate::error::IndexError;
use crate::note::Note;
use crate::utils::{normalize_data, resample_data};


#[derive(Clone)]
pub struct InstrumentTrack {
    data: Vec<f32>,
    sample_rate: u32,
    volume: f32,
}

impl InstrumentTrack {
    pub fn new(sample_rate: u32, volume: f32) -> Self {
        Self {
            data: vec![],
            sample_rate,
            volume,
        }
    }

    pub fn data(&self) -> Vec<f32> {
        normalize_data(self.data.clone(), self.volume)
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn add_note(&mut self, note: Note) {
        let data = resample_data(note.data().clone(), note.sample_rate(), self.sample_rate);
        self.data.extend(data.iter());
    }
}

pub struct Mixdown {
    tracks: Vec<Vec<InstrumentTrack>>,
    channel: u16,
    sample_rate: u32,
}

impl Mixdown {
    const BIT_PER_SAMPLE: u16 = 32;
    const SAMPLE_FORMAT: SampleFormat = SampleFormat::Float;

    pub fn new(channel: u16, sample_rate: u32) -> Self {
        Self {
            tracks: vec![vec![]; channel as usize],
            channel,
            sample_rate,
        }
    }

    pub fn channel(&self) -> u16 {
        self.channel
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channel_data(&self, channel: u16) -> Result<Vec<f32>, Box<dyn Error>> {
        match self.tracks.get(channel as usize) {
            Some(channel_track) => {
                let mut track_data_list = vec![];
                let mut volume = 0f32;
                for track in channel_track.iter() {
                    track_data_list.push(resample_data(track.data.clone(), track.sample_rate, self.sample_rate));
                    if volume < track.volume {
                        volume = track.volume;
                    }
                }
                let length = track_data_list.iter().map(|data| data.len()).max().or(Some(0usize)).unwrap();

                let mut data = vec![0f32; length];
                for track_data in track_data_list.iter() {
                    for (i, &d) in track_data.iter().enumerate() {
                        data[i] += d;
                    }
                }

                Ok(normalize_data(data, volume))
            },
            None => Err(Box::new(IndexError::new(channel as usize, self.channel as usize))),
        }
    }

    pub fn data(&self) -> Result<Vec<f32>, Box::<dyn Error>> {
        let mut channel_data_list = vec![];
        for channel in 0..self.channel {
            let channel_data = self.channel_data(channel)?;
            channel_data_list.push(channel_data);
        }
        let length = channel_data_list.iter().map(|channel_data| channel_data.len()).max().or(Some(0usize)).unwrap();
        let mut data = vec![0f32; length * self.channel as usize];
        for (channel_id, channel_data) in channel_data_list.iter().enumerate() {
            for (i, &d) in channel_data.iter().enumerate() {
                data[self.channel as usize * i + channel_id] += d;
            }
        }

        Ok(data)
    }

    pub fn add_track(&mut self, channel: u16, track: InstrumentTrack) -> Result<(), Box<dyn Error>> {
        match self.tracks.get_mut(channel as usize) {
            Some(channel_track) => {
                channel_track.push(track);
                Ok(())
            },
            None => Err(Box::new(IndexError::new(channel as usize, self.channel as usize))),
        }
    }

    pub fn save(&self, filename: PathBuf) -> Result<(), Box::<dyn Error>> {
        let spec_out = WavSpec {
            channels: self.channel,
            sample_rate: self.sample_rate,
            bits_per_sample: Self::BIT_PER_SAMPLE,
            sample_format: Self::SAMPLE_FORMAT,
        };
        let mut writer = WavWriter::create(filename, spec_out)?;
        let data = self.data()?;
        for s in data {
            writer.write_sample(s)?;
        }
        writer.finalize()?;

        Ok(())
    }
}

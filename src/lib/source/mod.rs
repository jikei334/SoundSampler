pub mod sampler;
pub mod sin;
pub mod triangle;

use crate::note::Note;
use crate::utils::resample_linear;


const FADE_RATE: f32 = 0.005f32;
const FADE_SECONDS_MIN: f32 = 0.002f32;

fn fit_length(note: Note, target_seconds: f32) -> Note {
    let target_len = (target_seconds * note.sample_rate() as f32) as usize;
    if note.data().is_empty() {
        return Note::new(vec![0f32; target_len], note.sample_rate());
    }
    if note.data().len() >= target_len {
        let mut data = note.data().clone();
        data.truncate(target_len);
        return Note::new(data, note.sample_rate());
    }

    let mut out = Vec::with_capacity(target_len);
    while out.len() < target_len {
        let remain = target_len - out.len();
        if remain >= note.data().len() {
            out.extend_from_slice(note.data());
        } else {
            out.extend_from_slice(&note.data()[..remain]);
        }
    }

    Note::new(out, note.sample_rate())
}

fn pitch_shift_semitones(note: Note, semitones: f32) -> Note {
    let factor = 2f32.powf(semitones / 12f32);

    Note::new(
        resample_linear(note.data().clone(), factor),
        note.sample_rate()
    )
}

pub fn fade_in_out(note: Note, fade_seconds: f32) -> Note {
    let fade_samples = (fade_seconds * note.sample_rate() as f32) as usize;
    let n = note.data().len();
    let mut data = note.data().clone();
    for i in 0..fade_samples.min(n) {
        let g = i as f32 / fade_samples.max(1) as f32;
        data[i] *= g;
    }
    for i in 0..fade_samples.min(n) {
        let g = i as f32 / fade_samples.max(1) as f32;
        data[n-1-i] *= g;
    }

    Note::new(data, note.sample_rate())
}

pub trait SoundSource {
    fn get_base(&self) -> Note;

    fn get_rest(&self) -> Note {
        Note::new(
            vec![0f32],
            self.sample_rate()
        )
    }

    fn sample_rate(&self) -> u32 {
        self.get_base().sample_rate()
    }

    fn get_note(&self, seconds: f32, semitones: Option<f32>) -> Note {
        let note = match semitones {
            Some(semitone) => {
                let note = self.get_base();
                pitch_shift_semitones(note, semitone)
            },
            None => self.get_rest(),
        };
        let note = fit_length(note, seconds);
        let fade_seconds = (seconds * FADE_RATE).max(FADE_SECONDS_MIN);
        let note = fade_in_out(note, fade_seconds);

        note
    }
}

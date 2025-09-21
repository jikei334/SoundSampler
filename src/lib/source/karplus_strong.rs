use rand;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};

use crate::note::Note;
use crate::score::ScorePartSource;
use crate::source::SoundSource;


const DEFAULT_SAMPLE_RATE: u32 = 48_000;
const DEFAULT_DURATION_SECONDS: f32 = 2.0f32;
const FREQUENCY_C4: f32 = 261.6256f32;

fn create_karpus_strong_note(frequency: f32, duration: f32, sample_rate: u32, rng: &mut ChaCha8Rng) -> Note {
    let num_samples = (duration * sample_rate as f32) as usize;
    let buffer_size = (sample_rate as f32 / frequency) as usize;

    let mut buffer: Vec<f32> = (0..buffer_size)
        .map(|_| rng.next_u64() as f32 / u64::MAX as f32 * 2.0 - 1.0)
        .collect();

    let mut output = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let sample = buffer[i % buffer_size];
        output.push(sample);

        let next = 0.5 * (buffer[i % buffer_size] + buffer[(i+1) % buffer_size]);
        buffer[i % buffer_size] = next;
    }

    Note::new(output, sample_rate, None)
}

pub struct KarpusStrong {
    seed: Option<u64>,
    note: Note,
}

impl KarpusStrong {
    pub fn new(seed: Option<u64>) -> Self {
        let mut rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_seed(Default::default()),
        };
        let note = create_karpus_strong_note(FREQUENCY_C4, DEFAULT_DURATION_SECONDS, DEFAULT_SAMPLE_RATE, &mut rng);

        Self {
            seed,
            note,
        }
    }
}

impl SoundSource for KarpusStrong {
    fn get_base(&self) -> Note {
        self.note.to_owned()
    }

    fn to_score_part_source(&self) -> ScorePartSource {
        ScorePartSource::KarpusStrong(self.seed)
    }
}

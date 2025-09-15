use crate::note::Note;


#[derive(Clone)]
pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
        }
    }

    pub fn apply(&self, note: Note) -> Note {
        let mut data = note.data().iter().enumerate().map(|(i, &d)| {
            let t = i as f32 / note.sample_rate() as f32;
            d * if t < self.attack {
                t / self.attack
            } else if t < self.attack + self.decay {
                1f32 + (self.sustain - 1f32) * (t - self.attack) / self.decay
            } else {
                self.sustain
            }
        }).collect::<Vec<_>>();
        for i in 0..(note.sample_rate() as f32 * self.release) as usize {
            let d = note.data()[i as usize % note.data().len()];
            data.push(d * self.sustain);
        }

        Note::new(data, note.sample_rate(), note.envelope().clone())
    }
}

#[derive(Clone)]
pub struct Note {
    data: Vec<f32>,
    sample_rate: u32,
}

impl Note {
    pub fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        Note {
            data,
            sample_rate,
        }
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

use crate::envelope::Envelope;


#[derive(Clone)]
pub struct Note {
    data: Vec<f32>,
    sample_rate: u32,
    envelope: Option<Envelope>,
}

impl Note {
    pub fn new(data: Vec<f32>, sample_rate: u32, envelope: Option<Envelope>) -> Self {
        Note {
            data,
            sample_rate,
            envelope,
        }
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn envelope(&self) -> &Option<Envelope> {
        &self.envelope
    }
}

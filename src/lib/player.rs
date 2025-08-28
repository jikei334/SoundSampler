use std::error::Error;

use rodio::{buffer, OutputStreamBuilder, Sink};
use rodio::stream::OutputStream;

use crate::track::Mixdown;


pub struct Player {
    // If this lacks, mixer will be dropped before playing and no sounds will be played.
    #[allow(dead_code)]
    stream_handle: OutputStream,
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let stream_handle = OutputStreamBuilder::open_default_stream()
            .expect("failed to open audio stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        Self {
            stream_handle,
            sink,
        }
    }

    pub fn add_mixdown(&mut self, mixdown: Mixdown) -> Result<(), Box::<dyn Error>> {
        let source = buffer::SamplesBuffer::new(mixdown.channel(), mixdown.sample_rate(), mixdown.data()?);
        self.sink.append(source);

        Ok(())
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end()
    }
}

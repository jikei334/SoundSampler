use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

use clap::Parser;

use lib::player::Player;
use lib::score::Score;
use lib::track::Mixdown;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // Path to src .wav file
    src_file: PathBuf,

    // sample rate of output
    #[arg(long)]
    sample_rate: Option<u32>,

    // volume
    #[arg(long, default_value_t = 1.0f32)]
    volume: f32,

    // Path to output .wav file
    // if none, play without saving
    #[arg(short, long)]
    out_file: Option<PathBuf>,

    // BPM
    #[arg(long, default_value_t = 100.0)]
    bpm: f32,

    // beats per note
    #[arg(long, default_value_t = 0.5)]
    beats_per_note: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let json = read_to_string(args.src_file)?;
    let score: Score = serde_json::from_str(&json)?;
    let song: Result<Mixdown, Box<dyn Error>> = score.into();
    let song = song?;

    match args.out_file {
        Some(out_file) => {
            song.save(out_file)?;
        },
        None => {
            let mut player = Player::new();
            player.add_mixdown(song)?;
            player.sleep_until_end();
        },
    }

    Ok(())
}

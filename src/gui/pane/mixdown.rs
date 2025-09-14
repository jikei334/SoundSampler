use std::error::Error;
use std::path::{Path, PathBuf};

use eframe::egui;
use egui_file::FileDialog;

use lib::track::Mixdown;
use lib::player::Player;
use lib::score::Score;

use crate::pane::Pane;
use crate::pane::track::TrackPane;


pub struct MixdownPane {
    num_channel: u16,
    sample_rate: u32,
    track_panes: Vec<TrackPane>,
    player: Player,
    source_json_file: Option<PathBuf>,
    save_file_dialog: Option<FileDialog>,
}

impl MixdownPane {
    const MAX_CHANNEL: u16 = 32;

    const DEFAULT_NUM_CHANNEL: u16 = 1;
    const DEFAULT_SAMPLE_RATE: u32 = 44100;

    pub fn new(score: Score, source_json_file: Option<PathBuf>) -> Result<Self, Box<dyn Error>> {
        let mut track_panes = Vec::new();
        for track in score.tracks() {
            track_panes.push(TrackPane::from_score_part(track, score.num_channel())?);
        }
        Ok(Self {
            num_channel: score.num_channel(),
            sample_rate: score.sample_rate(),
            track_panes,
            player: Player::new(),
            source_json_file,
            save_file_dialog: None,
        })
    }

    pub fn source_json_file(&self) -> &Option<PathBuf> {
        &self.source_json_file
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let score: Score = (&*self).into();
        let json = serde_json::to_string_pretty(&score)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    pub fn save_as(&mut self, ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        let score: Score = (&*self).into();
        if let Some(dialog) = &mut self.save_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(path) = dialog.path() {
                    let json = serde_json::to_string_pretty(&score)?;
                    std::fs::write(path, json)?;
                    self.source_json_file = Some(path.to_path_buf());
                    self.save_file_dialog = None;
                }
            }
        }

        Ok(())
    }
}

impl Pane for MixdownPane {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let play_or_pause_text = if self.player.is_empty() || self.player.is_paused() {
            "Play"
        } else {
            "Pause"
        };
        if ui.button(play_or_pause_text).clicked() {
            if self.player.is_empty() || self.player.is_paused() {
                if self.player.is_empty() {
                    let score: Score = (&*self).into();
                    match Result::<Mixdown, Box<dyn Error>>::from(score) {
                        Ok(mixdown) => {
                            match self.player.add_mixdown(mixdown) {
                                Ok(()) => (),
                                Err(error) => {
                                    println!("{:?}", error);
                                },
                            }
                        },
                        Err(error) => {
                            println!("{:?}", error);
                        }
                    }
                }
                self.player.play();
            } else {
                self.player.pause();
            }
        }
        ui.horizontal(|ui| {
            ui.label("MAX Channel");
            ui.add(egui::DragValue::new(&mut self.num_channel)
                .range(0u16..=Self::MAX_CHANNEL));

            ui.label("sample rate");
            ui.add(egui::DragValue::new(&mut self.sample_rate)
                .range(0u32..=u32::MAX));

            if ui.button("Add Track").clicked() {
                self.track_panes.push(TrackPane::new(self.num_channel));
            }
        });
        for (i, track_pane) in self.track_panes.iter_mut().enumerate() {
            track_pane.set_max_channel(self.num_channel);
            let title = format!("Track {}", i+1);
            egui::Window::new(title)
                .collapsible(true)
                .resizable(true)
                .show(ctx, |ui| {
                    track_pane.ui(ui, ctx);
                });
        }
    }
}

impl Default for MixdownPane {
    fn default() -> Self {
        Self {
            num_channel: Self::DEFAULT_NUM_CHANNEL,
            sample_rate: Self::DEFAULT_SAMPLE_RATE,
            track_panes: vec![TrackPane::new(Self::DEFAULT_NUM_CHANNEL)],
            player: Player::new(),
            source_json_file: None,
            save_file_dialog: None,
        }
    }
}

impl From<&MixdownPane> for Score {
    fn from(mixdown_pane: &MixdownPane) -> Self {
        Self::new(
            mixdown_pane.num_channel,
            mixdown_pane.sample_rate,
            mixdown_pane.track_panes.iter().map(|track_pane| track_pane.into()).collect(),
        )
    }
}

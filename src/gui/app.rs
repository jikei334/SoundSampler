use std::error::Error;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use eframe::egui;
use egui::RichText;
use egui_file::FileDialog;

use lib::score::Score;

use crate::pane::Pane;
use crate::pane::mixdown::MixdownPane;


enum Message {
    Info(String),
    Warning(String),
    Error(String),
}

pub struct SoundSampler {
    json_path: Option<PathBuf>,
    json_path_dialog: Option<FileDialog>,
    message: Option<Message>,
    mixdown_pane: Option<MixdownPane>,
    save_file_dialog: Option<FileDialog>,
}

impl Default for SoundSampler {
    fn default() -> Self {
        Self {
            json_path: None,
            json_path_dialog: None,
            message: None,
            mixdown_pane: Some(MixdownPane::default()),
            save_file_dialog: None,
        }
    }
}

impl SoundSampler {
    fn load_json(&mut self, json_file: PathBuf) -> Result<(), Box<dyn Error>> {
        let opened_json_file = read_to_string(json_file.to_owned())?;
        let score: Score = serde_json::from_str(&opened_json_file)?;
        self.mixdown_pane = Some(MixdownPane::new(score, Some(json_file))?);

        Ok(())
    }
}

impl eframe::App for SoundSampler {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu")
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            self.mixdown_pane = Some(MixdownPane::default());
                        }
                        if ui.button("Open").clicked() && self.json_path_dialog.is_none() {
                            let filter = Box::new({
                                let ext = Some(OsStr::new("json"));
                                move |path: &Path| -> bool { path.extension() == ext }
                            });
                            let mut dialog = FileDialog::open_file(self.json_path.clone()).show_files_filter(filter);
                            dialog.open();
                            self.json_path_dialog = Some(dialog);
                        }

                        if let Some(mixdown_pane) = self.mixdown_pane.as_mut() {
                            if let Some(source_json_file) = mixdown_pane.source_json_file().to_owned() {
                                if ui.button("Save").clicked() {
                                    if let Err(error) = mixdown_pane.save(&source_json_file) {
                                        println!("{:?}", error);
                                    }
                                }
                            }
                            if ui.button("Save As ..").clicked() {
                                let mut dialog = FileDialog::save_file(mixdown_pane.source_json_file().to_owned())
                                    .show_files_filter(
                                    Box::new(|path: &Path| path.extension().map(|ext| ext == "json").unwrap_or(false))
                                );
                                dialog.open();
                                self.save_file_dialog = Some(dialog);
                            }
                            match mixdown_pane.save_as(&ctx) {
                                Ok(()) => (),
                                Err(error) => {
                                    println!("{:?}", error);
                                },
                            }
                        }
                    })
                });

                let mut json_path = None;

                if let Some(dialog) = &mut self.json_path_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            json_path = Some(file.to_owned().to_path_buf());
                            self.json_path_dialog = None;
                        }
                    }
                };

                if let Some(json_path) = json_path {
                    match self.load_json(json_path.to_owned().to_path_buf().to_owned()) {
                        Ok(()) => {
                            self.message = Some(Message::Info("Loaded!".to_string()));
                        },
                        Err(error) => {
                            self.message = Some(Message::Error(format!("{:?}", error).to_string()));
                        },
                    };
                    ui.label(json_path.to_str().unwrap());

                    self.json_path = Some(json_path);
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(message) = &self.message {
                let message_ui = match message {
                    Message::Info(text) => RichText::new(text),
                    Message::Warning(text) => RichText::new(text),
                    Message::Error(text) => RichText::new(text),
                };
                ui.label(message_ui);
            }

            if let Some(mixdown_pane) = self.mixdown_pane.as_mut() {
                mixdown_pane.ui(ui, ctx);
            }
        });
    }
}

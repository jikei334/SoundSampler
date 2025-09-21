use eframe::egui;
use std::ffi::OsStr;
use std::path::Path;

use egui_file::FileDialog;

use lib::score::ScorePartSource;

use crate::pane::Pane;


#[derive(Clone, Copy, PartialEq)]
enum SoundSourceChoice {
    Sampler,
    Sin,
    Triangle,
    KarpusStrong,
}

impl SoundSourceChoice {
    fn to_str(&self) -> &'static str {
        match self {
            SoundSourceChoice::Sampler => "Sampler",
            SoundSourceChoice::Sin => "Sin",
            SoundSourceChoice::Triangle => "Triangle",
            SoundSourceChoice::KarpusStrong => "KarpusStrong",
        }
    }
}

impl From<ScorePartSource> for SoundSourceChoice {
    fn from(score_part_source: ScorePartSource) -> Self {
        match score_part_source {
            ScorePartSource::Sampler(_) => SoundSourceChoice::Sampler,
            ScorePartSource::Sin => SoundSourceChoice::Sin,
            ScorePartSource::Triangle => SoundSourceChoice::Triangle,
            ScorePartSource::KarpusStrong(_) => SoundSourceChoice::KarpusStrong,
        }
    }
}

pub struct SoundSourcePane {
    source: ScorePartSource,
    current: SoundSourceChoice,
    dialog: Option<FileDialog>,
}

impl SoundSourcePane {
    pub fn new(source: ScorePartSource) -> Self {
        Self {
            source: source.to_owned(),
            current: source.to_owned().into(),
            dialog: None,
        }
    }

    pub fn source(&self) -> ScorePartSource {
        self.source.to_owned()
    }
}

impl Pane for SoundSourcePane {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.label("Sound Source");
            let mut next = self.current;
            egui::ComboBox::from_label(match &self.source {
                ScorePartSource::Sin => "",
                ScorePartSource::Triangle => "",
                ScorePartSource::Sampler(file_name) => file_name.to_str().or(Some("")).unwrap(),
                ScorePartSource::KarpusStrong(seed) => "",
            })
                .selected_text(self.current.to_str())
                .show_ui(ui, |ui| {
                    let choices = vec![
                        SoundSourceChoice::Sin,
                        SoundSourceChoice::Triangle,
                        SoundSourceChoice::Sampler,
                        SoundSourceChoice::KarpusStrong,
                    ];
                    for choice in choices {
                        ui.selectable_value(&mut next, choice, choice.to_str());
                    }
                });
            if next != self.current {
                match next {
                    SoundSourceChoice::Sampler => {
                        let filter = Box::new({
                            let ext = Some(OsStr::new("wav"));
                            move |path: &Path| -> bool { path.extension() == ext }
                        });
                        let mut dialog = FileDialog::open_file(None).show_files_filter(filter);
                        dialog.open();
                        self.dialog = Some(dialog);
                    },
                    SoundSourceChoice::Sin => {
                        self.current = next;
                        self.source = ScorePartSource::Sin;
                    },
                    SoundSourceChoice::Triangle => {
                        self.current = next;
                        self.source = ScorePartSource::Triangle;
                    },
                    SoundSourceChoice::KarpusStrong => {
                        self.current = next;
                        self.source = ScorePartSource::KarpusStrong(None);
                    },
                }
            }

            if let Some(dialog) = &mut self.dialog {
                if dialog.show(ctx).selected() {
                    if let Some(wav_file) = dialog.path() {
                        self.current = next;
                        self.source = ScorePartSource::Sampler(wav_file.to_owned().to_path_buf());
                        self.dialog = None;
                    }
                }
            }
        });
    }
}

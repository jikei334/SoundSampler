pub mod mixdown;
pub mod sound_source;
pub mod track;

use eframe::egui;

pub trait Pane {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
}

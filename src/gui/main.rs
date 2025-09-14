#![windows_subsystem = "windows"]

mod app;
mod pane;

use std::error::Error;

use eframe::{egui, NativeOptions};
use eframe::egui::viewport::IconData;

use app::SoundSampler;


fn load_icon() -> Result<IconData, Box<dyn Error>> {
    let bytes = include_bytes!("../../assets/icon.png");
    let icon_image = image::load_from_memory(bytes)?.into_rgba8();
    let (width, height) = icon_image.dimensions();
    let rgba = icon_image.into_raw();
    Ok(IconData {
        rgba,
        width,
        height,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let icon = load_icon()?;
    let option = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon),
        ..Default::default()
    };
    Ok(eframe::run_native(
        "Sound Sampler",
        option,
        Box::new(|_| {
            Ok(Box::<SoundSampler>::default())
        }),
    )?)
}

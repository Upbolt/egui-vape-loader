#![windows_subsystem = "windows"]
#![feature(variant_count)]

use eframe::epaint::Vec2;
use vape::Vape;

mod vape;
mod widgets;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.decorated = false;
    options.transparent = true;
    options.initial_window_size = Some(Vec2 { x: 820., y: 480. });

    eframe::run_native("Vape", options, Box::new(|cc| Vape::new(cc))).expect("egui failure");
}

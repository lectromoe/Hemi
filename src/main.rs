#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![allow(unused, dead_code)]

mod app;
mod keyboard;
use app::App;
use eframe::egui::Vec2;
use eframe::{egui::{Style, Visuals}, epaint::Rounding};
use std::sync::Arc;

pub struct StyleConfig {
    button_size: f32,
    button_indent: f32,
    button_spacing: Vec2,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            button_size: 75.,
            button_indent: 35.,
            button_spacing: Vec2::new(10., 10.),
        }
    }
}
pub struct ApplicationConfig {
    style: StyleConfig,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        ApplicationConfig {
            style: StyleConfig::default(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {


    let app = App::default();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1000.0, 800.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "Hemi Typer", 
        native_options, 
        Box::new(|cc| {
            #[cfg(feature = "persistence")]
            if let Some(storage) = _storage {
                *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
            }

            let app_style = Style {
                override_text_style: None,
                wrap: None,
                spacing: Default::default(),
                interaction: Default::default(),
                visuals: Visuals {
                    window_rounding: Rounding { nw: 0., ne: 0., sw: 0., se: 0. },
                    ..Default::default()
                },
                ..Default::default()
            };

            cc.egui_ctx.set_style(Arc::new(app_style));

        Box::new(app)
    }),
);
}

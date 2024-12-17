#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod config;
mod ui;
mod feature;
mod font;

use eframe::egui;
use eframe::egui::IconData;

fn main() -> eframe::Result {

    let icon_data = include_bytes!("./icons/icon.png");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
    let rgba_data = img.into_rgba8();
    let (w,h)=(rgba_data.width(),rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_inner_size( [1400.0, 700.0])
            .with_resizable(false)
            .with_maximized(false)
            .with_fullscreen(false)
            .with_maximize_button(false)
            .with_icon(IconData { rgba:  raw_data, width: w, height: h })
        // .with_decorations(false)  //头顶导航栏
        ,
        centered:true,
        ..Default::default()
    };

    eframe::run_native(
        "桌面app",
        options,
        Box::new(|cc| Ok(Box::new(ui::home::layout::ApplicationComponent::new(cc)))),
    )
}


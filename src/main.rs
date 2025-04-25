#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod config;
mod ui;
mod font;
mod win32;
use eframe::egui;
use eframe::egui::IconData;
use config::logger;

#[tokio::main]
async fn main() ->eframe::Result {
    // logger::logger_init("./logs/").await;
    let icon_data = include_bytes!("./icons/icon.png");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
    let rgba_data = img.into_rgba8();
    let (w,h)=(rgba_data.width(),rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_inner_size( [1000.0, 500.0])
            .with_resizable(false)
            .with_maximized(false)
            .with_fullscreen(false)
            .with_maximize_button(false)
            .with_icon(IconData { rgba:  raw_data, width: w, height: h })
            .with_decorations(true)  //头顶导航栏是是否显示
           .with_visible(true)
           .with_decorations(true)
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


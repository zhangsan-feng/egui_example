

mod config;
mod gui;
mod font;
mod component;

use eframe::egui;
use eframe::egui::IconData;
use eframe::egui_wgpu::winit;

use eframe::wgpu::wgc::command::bundle_ffi::wgpu_render_bundle_insert_debug_marker;
use config::logger;

/*
https://aws.github.io/aws-lc-rs/requirements/windows.html

Download the Build Tools for Visual Studio installer.
Download Windows CMake Installer
Download and install the Netwide Assembler (NASM)
Download and install Ninja
Download LLVM Installer
*/



#[tokio::main]
async fn main() ->eframe::Result {
    
    // logger::logger_init("./logs/").await;
    let icon_data = include_bytes!("./icons/icon.png");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
    let rgba_data = img.into_rgba8();
    let (w,h)=(rgba_data.width(),rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();

    let options = eframe::NativeOptions {
        // renderer: eframe::Renderer::Wgpu,
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size( [1400.0, 800.0])
            .with_icon(IconData { rgba:  raw_data, width: w, height: h })
            
            // .with_maximized(true)
        ,
        centered:true,
        ..Default::default()
    };

    
    eframe::run_native(
        "egui_example",
        options,
        Box::new(|cc| Ok(Box::new(gui::layout::ApplicationComponent::new(cc)))),
    )
}


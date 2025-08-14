use std::hash::Hash;
use eframe::emath::vec2;
use egui::Rangef;
use serde::{Serialize, Deserialize};
use crate::font::font;
use crate::gui::component::{app_manager, network_manager, process_manager, system_manager};
use crate::gui::navigation::navigation::Navigation;


#[derive(Default)]
pub struct ApplicationComponent {
    top_nav: Navigation,
    left_panel_width: f32,

}

impl ApplicationComponent {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        font::replace_fonts(&cc.egui_ctx);
        font::add_font(&cc.egui_ctx);
        Self {
            top_nav: Navigation::new(),
            left_panel_width: 120.0,

        }
    }
}

impl eframe::App for ApplicationComponent {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {

    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(5))
                    .show(ui, |ui| {
                self.top_nav.view(ctx, ui)
            });
        });


        egui::SidePanel::left("left_panel").resizable(true)
            .default_width(self.left_panel_width)
            .width_range(150.0..=400.0)
            .show(ctx, |ui| {
            egui::Frame::default()
                .inner_margin(egui::Margin::same(5))
                .show(ui, |ui| {ui.vertical(|ui| {
                // ui.style_mut().spacing.button_padding =  vec2(8.0, 8.0);
                //
        
                    
                    ui.label("会话");
                    ui.add_space(5.0);
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                            for i in 1..=20 {
                                ui.horizontal(|ui| {
                                    ui.label(format!("选项 {}", i));
                                    if ui.button("按钮").clicked() {
                                        println!("点击了选项 {}", i);
                                    }
                                });
                                ui.separator();
                            }
                        });


                });
            });
        });


        // let my_value = ctx.data_mut(|d| {
        //     d.get_persisted::<String>(egui::Id::from("my_key"))
        //         .unwrap_or_default()
        // });
        // println!("my_value: {:?}", my_value);
        // ctx.data_mut(|d| {
        //     d.insert_persisted(egui::Id::from("my_key"), "Hello World".to_string());
        // });



        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::default()
                .inner_margin(egui::Margin::same(5))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {});
                        ui.add_space(10.0);
                        ui.separator();
                        ui.horizontal(|ui| {
                            // match self.current_page {
                            //     ApplicationPage::ProcessManagerPage => {
                            //         process_manager::ProcessManager::new().render(ui)
                            //     }
                            //     ApplicationPage::SystemManagerPage => {
                            //         system_manager::SystemManager::new().render(ui)
                            //     }
                            //     ApplicationPage::AppManagerPage => {
                            //         app_manager::AppManager::new().render(ui)
                            //     }
                            //     ApplicationPage::NetworkManagerPage => {
                            //         network_manager::NetworkManager::new().render(ui)
                            //     }
                            // }
                        });
                    })

                });
        });


    }
}

/*

// 存储数据
ctx.data_mut(|d| {
    d.insert_temp(egui::Id::new("my_key"), "Hello World".to_string());
    d.insert_temp(egui::Id::new("number"), 42i32);
    d.insert_temp(egui::Id::new("color"), egui::Color32::RED);
});

// 读取数据
let text = ctx.data(|d| {
    d.get_temp::<String>(egui::Id::new("my_key"))
        .unwrap_or_default()
});

*/
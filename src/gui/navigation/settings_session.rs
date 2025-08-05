use eframe::emath::Align;
use egui::{InnerResponse, Stroke};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize)]
pub enum SessionType {
    #[default]
    SSH,
    Telnet,
    Serial
}

#[derive(Default)]
pub struct SettingsSession {
    is_down: bool,
    current_page: SessionType,
    input_msg:String,
}

impl SettingsSession {
    pub fn new() -> Self {
        Self {
            is_down: false,
            current_page: SessionType::SSH,
            input_msg: "".to_string(),
        }
    }

    pub fn view(
        &mut self,
        ctx: &egui::Context,
        state: &mut bool,
    ) -> Option<InnerResponse<Option<()>>> {
        egui::Window::new("123")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -50.0])
            .fixed_size([800.0, 600.0])
            .resizable(false)
            .movable(false)
            .collapsible(false)

            .frame(egui::Frame {
                fill: egui::Color32::WHITE,
                stroke: egui::Stroke::NONE,
                inner_margin: egui::Margin::same(15),
                outer_margin: egui::Margin::same(15),
                shadow: egui::epaint::Shadow::NONE,
                corner_radius: egui::CornerRadius::same(5),
            })
            .show(ctx, |ui| {


                ui.horizontal(|ui| {
                    ui.label("新建会话:");
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("x").clicked() {
                            *state = false;
                        };
                    })
                });
                ui.separator();

                let pages = [
                    (SessionType::SSH,   "SSH"),
                    (SessionType::Telnet,"Telnet"),
                    (SessionType::Serial, "Serial"),

                ];


                ui.horizontal(|ui| {

                    for (page, label) in pages.iter() {
                        let response = ui.add_sized(
                            [150.0, 30.0],
                            egui::Button::selectable(self.current_page == *page, *label)
                        );

                        if response.clicked() {
                            self.current_page = *page;
                        }
                        ui.add_space(4.0);
                    }

                });

              ui.horizontal(|ui| {
                        ui.set_height(500.0);
                        match self.current_page {
                            SessionType::SSH => {
                                ui.vertical(|ui| {
                                    ui.set_width(200.0);  
                                    ui.label("左侧");
                                });


                                ui.separator();

                                ui.vertical(|ui| {
                                    ui.set_width(200.0);  
                                    ui.label("右边");
                                });


                            }
                            SessionType::Telnet => {
                                ui.label("Telnet 会话配置");
                                // Telnet 相关的 UI
                            }
                            SessionType::Serial => {
                                ui.label("Serial 会话配置");
                                // Serial 相关的 UI
                            }
                        }

                    });
                ui.horizontal(|ui| {
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            if ui.button("取消").clicked() {
                                *state = false;
                            }
                            if ui.button("创建").clicked() {
                                *state = false;
                            }
                        },
                    );
                });
            })
    }
}

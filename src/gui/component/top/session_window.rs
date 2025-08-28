use std::fmt;
use eframe::emath::{Align};
use egui::{InnerResponse};
use serde::Serialize;
use uuid::Uuid;
use crate::gui::component::top::ssh_component::SshComponent;
use crate::gui::layout::Store;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize)]
pub enum SessionType {
    #[default]
    SSH,
    Telnet,
    Serial
}

#[derive(Default, Debug)]
pub struct SettingsSession {
    current_page: SessionType,
    position: Option<egui::Pos2>,
    first_show: bool,
    ssh_component: SshComponent,
    window_bg_color: egui::Color32,

}



impl SettingsSession {
    pub fn new() -> Self {
        Self {
            current_page:SessionType::SSH,
            position: None,
            first_show: true,
            ssh_component: SshComponent::new(),
            window_bg_color: egui::Color32::WHITE,
        }
    }

    pub fn view(&mut self, ctx: &egui::Context, state: &mut bool, store:&mut Store) -> Option<InnerResponse<Option<()>>> {
        let mut window = egui::Window::new("settings_session")
            .title_bar(false)
            .fixed_size([800.0, 600.0])
            .resizable(false)
            .movable(true)
            .collapsible(false)
            .frame(egui::Frame {
                fill: self.window_bg_color,
                stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                inner_margin: egui::Margin::same(15),
                outer_margin: egui::Margin::same(15),
                shadow: egui::epaint::Shadow{
                    offset: [1; 2],
                    blur: 8,
                    spread: 2,
                    color: egui::Color32::from_black_alpha(60),
                },
                corner_radius: egui::CornerRadius::same(5),
            });


        if self.first_show {
            let screen_rect = ctx.screen_rect();
            let window_size = egui::Vec2::new(800.0, 600.0);
            let center_pos = egui::Pos2::new(
                (screen_rect.width() - window_size.x) * 0.5,
                (screen_rect.height() - window_size.y) * 0.5 - 50.0  // 向上偏移50像素
            );
            self.position = Some(center_pos);
            self.first_show = false;
        }
        self.window_bg_color = ctx.style().visuals.window_fill;
        if let Some(pos) = self.position {
            window = window.current_pos(pos);
        }

        window.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("新建会话:");
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("x").clicked() {
                        *state = false;
                    };
                })
            });
            ui.separator();
            ui.add_space(5.0);

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

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.set_height(500.0);
                match self.current_page {
                    SessionType::SSH => {
                        self.ssh_component.view(ctx, ui)
                    }
                    SessionType::Telnet => {
                        ui.label("Telnet 会话配置")
                        // Telnet 相关的 UI
                    }
                    SessionType::Serial => {
                        ui.label("Serial 会话配置")
                        // Serial 相关的 UI
                    }
                }
            });
            ui.spacing();
            ui.horizontal(|ui| {
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if ui.button("取消").clicked() {
                            *state = false;
                        }
                        if ui.button("创建").clicked() {
                   
                            store.session.push(SshComponent{
                                ssh_username: self.ssh_component.ssh_username.clone(),
                                ssh_password: self.ssh_component.ssh_password.clone(),
                                ssh_port: self.ssh_component.ssh_port.clone(),
                                ssh_host: self.ssh_component.ssh_host.clone(),
                                id:Uuid::new_v4(),
                            });
              
                            *state = false;
                        }
                    },
                );
            });
        }).map(|inner_response| {
            // 保存窗口位置，这样下次打开时就会在同一位置
            let rect = inner_response.response.rect;
            self.position = Some(rect.min);
            inner_response
        })
    }

}
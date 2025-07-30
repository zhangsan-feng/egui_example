use eframe::emath::Align;
use egui::InnerResponse;

#[derive(Default)]
pub struct SettingsColor {
    selected_color: egui::Color32,
}


impl SettingsColor {
    
    pub fn new()->Self{
        Self{
            selected_color: egui::Color32::WHITE,
        }
    }

    pub fn view(& mut self, ctx: &egui::Context,  state: &mut bool) -> Option<InnerResponse<Option<()>>> {
        egui::Window::new("456")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -50.0])
            .default_size([400.0, 300.0])
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                
                ui.vertical(|ui| {
            
                    ui.horizontal(|ui| {
                        ui.label("颜色选择");
                        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui|{
                            if ui.button("×").clicked() {
                                *state = false;
                            };
                        })
                    });
                    ui.separator();

                    // 颜色选择器
                    ui.label("选择颜色:");
                    ui.color_edit_button_srgba(&mut self.selected_color);

                    ui.add_space(10.0);

                    // 显示当前选中的颜色
                    ui.horizontal(|ui| {
                        ui.label("当前颜色:");
                        let rect = ui.available_rect_before_wrap();
                        let color_rect = egui::Rect::from_min_size(
                            rect.min,
                            egui::Vec2::new(50.0, 20.0)
                        );
                        ui.painter().rect_filled(color_rect, 0.0, self.selected_color);
                        ui.allocate_space(egui::Vec2::new(60.0, 20.0));
                    });

                    ui.add_space(20.0);

                    ctx.style_mut(|style| {
                        style.visuals.window_fill = self.selected_color;
                        style.visuals.panel_fill = self.selected_color;
                        style.visuals.faint_bg_color = self.selected_color;
                        style.visuals.extreme_bg_color = self.selected_color;
                    });


                    // 按钮区域
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("取消").clicked() {
                                *state = false;
                            }

                            if ui.button("确定").clicked() {
                                // 应用颜色到整体背景

                                println!("选中的颜色: {:?}", self.selected_color);
                                *state = false;
                            }
                        });
                    });
                });
            })
    }
}
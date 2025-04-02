use egui::{self, vec2};  // 修改这里，添加 vec2 的导入
use super::layout::ApplicationPage;

pub struct Navigation;

impl Navigation {
    pub fn render(ui: &mut egui::Ui, current_page: &mut ApplicationPage) {
        ui.vertical(|ui| {
            // 设置背景颜色
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(240, 240, 240)
            ;
            // 设置按钮样式
            let button_padding = vec2(8.0, 8.0);
            ui.style_mut().spacing.button_padding = button_padding;
            
            // 设置最大宽度和对齐方式
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                // HomePage 菜单项
                if ui.selectable_label(
                    *current_page == ApplicationPage::HomePage,
                    "🏠 首页"
                ).clicked() {
                    *current_page = ApplicationPage::HomePage;
                }

                ui.add_space(4.0);

                // TestPage 菜单项
                if ui.selectable_label(
                    *current_page == ApplicationPage::TestPage,
                    "🔧 测试页面"
                ).clicked() {
                    *current_page = ApplicationPage::TestPage;
                }
            });
        });
    }
}
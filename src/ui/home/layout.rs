use eframe::emath::vec2;
use crate::font::font;
use crate::ui::component::{app_manager, network_manager, process_manager, system_manager};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ApplicationPage {
    #[default]
    ProcessManagerPage,
    SystemManagerPage,
    NetworkManagerPage,
    AppManagerPage,
}
#[derive(Default)]
pub struct ApplicationComponent {
    current_page:ApplicationPage
}

impl ApplicationComponent {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        font::replace_fonts(&cc.egui_ctx);
        font::add_font(&cc.egui_ctx);
        Self {
            current_page:ApplicationPage::ProcessManagerPage,
        }
    }
}

impl eframe::App for ApplicationComponent {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pages = [
            (ApplicationPage::ProcessManagerPage,   "进程管理"),
            (ApplicationPage::SystemManagerPage,    "系统信息"),
            (ApplicationPage::AppManagerPage,       "应用管理"),
            (ApplicationPage::NetworkManagerPage,   "网络管理"),
        ];

        egui::SidePanel::left("left_panel").resizable(false).max_width(120.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(240, 240, 240);
                let button_padding = vec2(8.0, 8.0);
                ui.style_mut().spacing.button_padding = button_padding;
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    for (page, label) in pages.iter() {
                        if ui.selectable_label(self.current_page == *page, *label).clicked() {
                            self.current_page = *page;
                        }
                        ui.add_space(4.0);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                ApplicationPage::ProcessManagerPage =>{
                    process_manager::ProcessManager::new().render(ui)
                }
                ApplicationPage::SystemManagerPage =>{
                    system_manager::SystemManager::new().render(ui)
                }
                ApplicationPage::AppManagerPage =>{
                    app_manager::AppManager::new().render(ui)
                }
                ApplicationPage::NetworkManagerPage =>{
                    network_manager::NetworkManager::new().render(ui)
                }
            }
        });

    }
}
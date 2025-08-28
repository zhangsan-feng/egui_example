use egui::{InnerResponse, Ui};

#[derive(Default)]
pub struct ExampleWindow {

}

impl ExampleWindow {
    pub fn view(&mut self, ctx: &egui::Context, ui:&mut egui::Ui) -> Option<InnerResponse<Option<()>>> {
        egui::Window::new("123")
            .title_bar(false)
            .fixed_size([800.0, 600.0])
            .resizable(false)
            .movable(true)
            .collapsible(false)

            .anchor(egui::Align2::CENTER_CENTER, [0.0, -50.0])
            .default_size([400.0, 300.0])
            .resizable(false)
            .movable(false)
            .frame(egui::Frame {
                fill: egui::Color32::WHITE,
                stroke: egui::Stroke::NONE,
                inner_margin: egui::Margin::same(15),
                outer_margin: egui::Margin::same(15),
                shadow: egui::epaint::Shadow::NONE,
                corner_radius: egui::CornerRadius::same(5),
            })
            .show(ctx, |ui| {})
    }
}
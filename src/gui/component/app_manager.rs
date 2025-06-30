pub struct AppManager {}


impl AppManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, ui:&mut egui::Ui) ->  egui::Response {
        ui.heading("AppManager")
    }
}
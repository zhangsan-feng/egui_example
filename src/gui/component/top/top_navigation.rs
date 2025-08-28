use egui::Align;
use crate::gui::component::top::color_window::SettingsColor;
use crate::gui::component::top::session_window::SettingsSession;
use crate::gui::layout::Store;

#[derive(Default)]
pub struct Navigation{
    show_new_session_window:bool,
    show_color_picker:bool,
    settings_color_component: SettingsColor,
    settings_session_component: SettingsSession 
}

impl Navigation {
    
    pub fn new() -> Self {
        Self{
            show_new_session_window:false,
            show_color_picker:false,
            settings_color_component:SettingsColor::new(),
            settings_session_component:SettingsSession::new(),
        }
    }

    
    pub fn view(& mut self, ctx: &egui::Context, ui:&mut egui::Ui, store:&mut Store)-> egui::Response{
        
        
        if self.show_new_session_window {
            self.settings_session_component.view(ctx, &mut self.show_new_session_window, store);
        }

        if self.show_color_picker {
            self.settings_color_component.view(ctx, &mut self.show_color_picker);
        }

        
        egui::MenuBar::new().ui(ui, |ui| {
            
            ui.menu_button("会话", |ui| {
                if ui.button("新会话").clicked() {
                    self.show_new_session_window = true;
                }
            });
            ui.menu_button("设置", |ui| {
                if ui.button("颜色设置").clicked(){
                    self.show_color_picker = true;
                }
            });

        }).response
    }
}
use egui::Align;
use crate::gui::component::top::color_window::SettingsColor;
use crate::gui::component::top::session_window::SessionType::SSH;
use crate::gui::component::top::session_window::SettingsSession;
use crate::gui::component::top::ssh_component::SshComponent;
use crate::gui::layout::Store;

#[derive(Default)]
pub struct Navigation{
    settings_color_component: SettingsColor,
    settings_session_component: SettingsSession
}

impl Navigation {
    
    pub fn new() -> Self {
        Self{

            settings_color_component:SettingsColor::new(),
            settings_session_component:SettingsSession::new(),
        }
    }

    
    pub fn view(& mut self, ctx: &egui::Context, ui:&mut egui::Ui, store:&mut Store)-> egui::Response{
        
        
        if store.show_new_session_window {
            self.settings_session_component.view(ctx, store);
        }

        if store.show_color_picker {
            self.settings_color_component.view(ctx, store);
        }

        
        egui::MenuBar::new().ui(ui, |ui| {
            
            ui.menu_button("会话", |ui| {
                if ui.button("新会话").clicked() {
                    store.is_editing_session = false;
                    self.settings_session_component.ssh_component = SshComponent{
                        ssh_username: "".to_string(),
                        ssh_password: "".to_string(),
                        ssh_port: "".to_string(),
                        ssh_host: "".to_string(),
                        id: Default::default(),
                    };
                    store.show_new_session_window = true;
                }
            });
            ui.menu_button("设置", |ui| {
                if ui.button("颜色设置").clicked(){
                    store.show_color_picker = true;
                }
            });

        }).response
    }
}
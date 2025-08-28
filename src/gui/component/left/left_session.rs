use std::collections::VecDeque;
use egui::scroll_area::ScrollSource;
use egui::ScrollArea;
use egui::Shape::Vec;
use uuid::Uuid;
use crate::gui::component::top::ssh_component::SshComponent;
use crate::gui::layout::{SessionContent, Store};

#[derive(Default)]
pub struct LeftSession{
    selected_ssh_index: Option<usize>,
    last_click_time: f64,
    last_clicked_index: Option<usize>,
}

impl LeftSession {
    
    pub fn new() -> Self {
        LeftSession{
            selected_ssh_index: None,
            last_click_time: 0.0,
            last_clicked_index: None,
        }
        
        
    }


    
    pub fn view(& mut self, ctx: &egui::Context, ui:&mut egui::Ui, store:&mut Store)-> egui::Response{

        ui.vertical(|ui| {
      
            ui.label("会话");
            ui.add_space(5.0);
            ui.separator();

    
            ScrollArea::vertical()
                .scroll_source(ScrollSource::NONE)
                .show_rows(
                    ui,
                    20.0,
                    store.session.len(), 
                    |ui, row_range| {
           
                        for row in row_range {
                            if let Some(session) = store.session.get(row) {
                                let is_selected = self.selected_ssh_index == Some(row);
                    
                                let response = ui.add_sized(
                                    [ui.available_width(), 20.0],
                                    egui::Button::selectable(is_selected, &session.ssh_host)
                                );

                       
                                if response.clicked() {
                                    let current_time = ui.input(|i| i.time);
                                
                                
                                    if let Some(last_index) = self.last_clicked_index {
                                        if last_index == row &&
                                            current_time - self.last_click_time < 0.2 {
                                            if let Some(session) = store.session.get(row) {
                                                println!("双击连接到: {}", session.ssh_host);
                                            }
                                            let mut new_session = session.clone();
                                            new_session.id = Uuid::new_v4();
                                            store.default_session = new_session.id;
                                            store.active_session.push(new_session);
                                            let session_content = SessionContent {
                                                history_input: VecDeque::new(),
                                                current_input: String::new(),
                                                prefix_input: String::from("root@localhost:~#"),
                                            };
                       
                                            store.session_content.insert(store.default_session, session_content);
                                      
                                        } else {
                                            self.selected_ssh_index = Some(row);
                                            println!("单击选中了项目: {}", row);
                                        }
                                    } else {
                                        self.selected_ssh_index = Some(row);
                                        println!("单击选中了项目: {}", row);
                                    }
                                
                                    self.last_click_time = current_time;
                                    self.last_clicked_index = Some(row);
                                }
                                
                          
                                response.context_menu(|ui| {
                                
                                    if let Some(session) = store.session.get(row) {
                                    
                                        if ui.add_sized([100.0, 20.0], egui::Button::new("编辑")).clicked() {
                                            println!("右键菜单: 编辑 {}", session.ssh_host);
                                            ui.close();
                                        }
                                        if ui.add_sized([100.0, 20.0], egui::Button::new("删除")).clicked() {
                                            
                                            println!("右键菜单: 删除 {}", session.ssh_host);
                                            store.session.remove(row);
                                            ui.close();
                                        }
                                    
                                    }
                                
                                });
                                ui.add_space(5.0)
                       
                            } 
                        }
                    }
                );
        }).response
    }
    
}
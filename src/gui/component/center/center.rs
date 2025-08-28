use std::collections::VecDeque;
use egui::CursorIcon::Text;
use egui::scroll_area::{ScrollBarVisibility, ScrollSource};
use egui::ScrollArea;
use crate::gui::layout::{SessionContent, Store};


#[derive(Default)]
pub struct Center{
  
}
const CTRL_A: u8 = 1;   // 行首
const CTRL_B: u8 = 2;   // 后退一个字符
const CTRL_C: u8 = 3;   // 中断 (SIGINT)
const CTRL_D: u8 = 4;   // EOF
const CTRL_E: u8 = 5;   // 行尾
const CTRL_L: u8 = 12;  // 清屏
const CTRL_Z: u8 = 26;  // 暂停 (SIGTSTP)

impl Center {
    pub fn new() -> Self {
        Center{
    
        }
    }

    pub fn view(& mut self, ctx: &egui::Context, ui:&mut egui::Ui, store:&mut Store)-> egui::Response{

        ui.vertical(|ui| {
            
            ui.horizontal(|ui| {
                ScrollArea::horizontal().id_salt(egui::Id::from("active_session")).show(ui, |ui| {
                    ui.set_height(35.0);
                        for data in store.active_session.clone().iter(){
                        // ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                ui.horizontal(|ui| {
                                    let is_selected = store.default_session == data.id;
                                    if  ui.add_sized([25.0, 25.0], egui::Button::new("x")).clicked(){
                                        if let Some(index) = store.active_session.iter().position(|session| session.id == data.id) {
                                            store.active_session.remove(index);
                                        }
                                        println!("删除session: {}", data.id);
                                    }
                                    let response = ui.add_sized([100.0, 25.0], egui::Button::selectable(is_selected, &data.ssh_host));
                                    if response.clicked(){ store.default_session = data.id; }
                                    ui.separator();
                                });
                            });

                        // });
                    }
          
                });
            });

    
            ui.separator();
            let mut display_height = ui.available_height();
            
            ui.vertical(|ui| {
                
                if let Some(session_content) = store.session_content.get(&store.default_session) {
                    ScrollArea::vertical().scroll_source(ScrollSource::NONE).
                    stick_to_bottom(true).animated(false).max_height(display_height - 30.0)
                        .id_salt(egui::Id::from("session_content"))
                        .show_rows(ui, 20.0, session_content.history_input.len(), |ui, row_range| {
                            for data in &session_content.history_input {
                                ui.set_min_width(ui.available_width());
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                                    ui.label(data);
                                    // ui.add_sized([ui.available_width(), 20.0], egui::Label::new(data));
                                });
                            }
                    });                     
                }
            });
            
            ui.horizontal(|ui| {
                if let Some(session_content) = store.session_content.get_mut(&store.default_session) {
                    ui.add_sized([100.0, 20.0], egui::Label::new(session_content.prefix_input.clone()));
                    if display_height < 50.0 { display_height = 50.0 }
                    ui.set_height(ui.available_height());

                    ui.vertical(|ui| { 
                    let text_edit_response = ui.add_sized(
                        [ui.available_width()-100.0, display_height],
                        egui::TextEdit::singleline(&mut
                             session_content.current_input)
                            .frame(false)
                            .clip_text(true)
                            .cursor_at_end(true)
                    );
                        

                    // let ctrl_c_pressed = ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C));

                    let enter_pressed = text_edit_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if enter_pressed {
                        session_content.history_input.push_back(format!("{} {}", session_content.prefix_input, session_content.current_input.clone()));
                        session_content.current_input.clear();
                        text_edit_response.request_focus();
                        
                    }
                    });
                }
            })

        }).response
    }
}
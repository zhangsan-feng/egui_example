use std::collections::VecDeque;
use std::net::TcpStream;
use egui::scroll_area::ScrollSource;
use egui::ScrollArea;
use egui::Shape::Vec;
use ssh2::Session;
use uuid::Uuid;
use crate::gui::component::top::ssh_component::SshComponent;
use crate::gui::layout::{SessionContent, Store};

#[derive(Default, Copy, Clone)]
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
                                                println!("双击连接到: {}:{}", session.ssh_host, session.ssh_port);
                                            }

                                            let mut new_session = session.clone();
                                            match TcpStream::connect(format!("{}:{}", new_session.ssh_host, new_session.ssh_port)) {
                                                Ok(tcp) => {
                                                    match Session::new() {
                                                        Ok(mut sess) => {
                                                            sess.set_tcp_stream(tcp);
                                                            match sess.handshake() {
                                                                Ok(()) => {
                                                                    match sess.userauth_password(&new_session.ssh_username, &new_session.ssh_password) {
                                                                        Ok(()) => {
                                                                            new_session.session_conn = Some(sess);
                                                                            new_session.id = Uuid::new_v4();
                                                                            store.default_session = new_session.id;
                                                                            store.active_session.push(new_session);

                                                                            let session_content = SessionContent {
                                                                                history_input: VecDeque::new(),
                                                                                current_input: String::new(),
                                                                                prefix_input: String::from("root@localhost:~#"),
                                                                            };

                                                                            store.session_content.insert(store.default_session, session_content);
                                                                            println!("SSH连接成功建立");
                                                                        }
                                                                        Err(e) => println!("用户认证失败: {}", e),
                                                                    }
                                                                }
                                                                Err(e) => println!("SSH握手失败: {}", e),
                                                            }
                                                        }
                                                        Err(e) => println!("创建SSH会话失败: {}", e),
                                                    }
                                                }
                                                Err(e) => println!("TCP连接失败: {}", e),
                                            }


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
                                            store.current_edit_session = session.clone();
                                            store.is_editing_session = true;
                                            store.show_new_session_window = true;
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
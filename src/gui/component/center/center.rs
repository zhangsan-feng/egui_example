use std::collections::VecDeque;
use std::io::Read;
use std::net::TcpStream;
use egui::CursorIcon::Text;
use egui::scroll_area::{ScrollBarVisibility, ScrollSource};
use egui::ScrollArea;
use ssh2::Session;
use crate::gui::layout::{SessionContent, Store};
use std::time::Duration;
use std::thread;

// ... 其他代码保持不变

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
                                        match store.active_session.get(index) {
                                            Some(rm) =>{ store.session_content.remove(&rm.id); }
                                            None =>{}
                                        }
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
                if let Some(session_content) = store.session_content.get_mut(&store.default_session) {
                    ScrollArea::vertical().id_salt(egui::Id::from("session_content"))
                        .show_rows(ui, 20.0, session_content.history_input.len(), |ui, row_range| {
                            for data in row_range {
                                match session_content.history_input.get(data) {
                                    Some(line) => {
                                        ui.set_width(ui.available_width());
                                        ui.allocate_ui_with_layout(
                                            egui::Vec2::new(ui.available_width(), 20.0),
                                            egui::Layout::left_to_right(egui::Align::LEFT),
                                            |ui| {
                                                ui.label(line);
                                            }
                                        );

                                    },
                                    None => {

                                    }
                                }
                            }
                            ui.horizontal(|ui|{
                                ui.label(session_content.prefix_input.clone());
                                let text_edit_response = ui.add_sized(
                                    [ui.available_width(), display_height],
                                    egui::TextEdit::singleline(&mut
                                        session_content.current_input)
                                        .id(egui::Id::new("input_field"))
                                        .frame(false)
                                        .clip_text(true)
                                        .cursor_at_end(true)
                                );

                                let enter_pressed = text_edit_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                                let tab_pressed = text_edit_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Tab));

                                if tab_pressed {
                                    // 添加输入到历史记录
                                    session_content.history_input.push_back(format!("{} {}", session_content.prefix_input, session_content.current_input.clone()));

                                    // 执行SSH命令
                                    let command = session_content.current_input.clone();
                                    session_content.current_input.clear();

                                    // 查找对应的active session
                                    if let Some(active_session) = store.active_session.iter_mut().find(|s| s.id == store.default_session) {
                                        if let Some(ref mut session) = active_session.session_conn {

                                            match self.execute_ssh_command(session, &command) {
                                                Ok(output) => {
                                                    println!("命令执行成功，输出: {}", output);
                                                    if output.trim().is_empty() {
                                                        session_content.history_input.push_back("(命令执行成功，无输出)".to_string());
                                                    } else {
                                                        for line in output.lines() {
                                                            session_content.history_input.push_back(line.to_string());
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("命令执行错误: {}", e);
                                                    session_content.history_input.push_back(format!("错误: {}", e));
                                                }
                                            }
                                        }
                                    }

                                    text_edit_response.request_focus();
                                    ctx.request_repaint(); // 强制重绘
                                }

                                if enter_pressed {
                                    // 添加输入到历史记录
                                    session_content.history_input.push_back(format!("{} {}", session_content.prefix_input, session_content.current_input.clone()));

                                    // 执行SSH命令
                                    let command = session_content.current_input.clone();
                                    session_content.current_input.clear();

                                    // 查找对应的active session
                                    if let Some(active_session) = store.active_session.iter_mut().find(|s| s.id == store.default_session) {
                                        if let Some(ref mut session) = active_session.session_conn {

                                            match self.execute_ssh_command(session, &command) {
                                                Ok(output) => {
                                                    println!("命令执行成功，输出: {}", output);
                                                    if output.trim().is_empty() {
                                                        session_content.history_input.push_back("(命令执行成功，无输出)".to_string());
                                                    } else {
                                                        for line in output.lines() {
                                                            session_content.history_input.push_back(line.to_string());
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("命令执行错误: {}", e);
                                                    session_content.history_input.push_back(format!("错误: {}", e));
                                                }
                                            }
                                        }
                                    }

                                    text_edit_response.request_focus();
                                    ctx.request_repaint(); // 强制重绘
                                }
                            });
                        });
                }

            });



        }).response
    }

    fn execute_ssh_command(&self, session: &Session, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("执行命令: {}", command);
        let mut channel = session.channel_session()?;

        channel.exec(command).expect("failed to execute command");

        let mut output = String::new();
        let mut stderr = String::new();
        let mut buffer = [0; 8192];

        // 循环读取数据，直到通道关闭
        loop {
            // 读取标准输出
            match channel.read(&mut buffer) {
                Ok(0) => break, // 没有更多数据
                Ok(n) => {
                    let chunk = String::from_utf8_lossy(&buffer[..n]);
                    output.push_str(&chunk);
                }
                Err(e) => {
                    // 如果是EAGAIN错误，表示暂时没有数据可读，继续等待
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                    break;
                }
            }
        }

        // 读取错误输出
        loop {
            match channel.stderr().read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let chunk = String::from_utf8_lossy(&buffer[..n]);
                    stderr.push_str(&chunk);
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                    break;
                }
            }
        }

        channel.send_eof()?;

        channel.wait_close()?;

        let exit_status = channel.exit_status()?;
        println!("命令退出状态: {}", exit_status);

        println!("标准输出长度: {}, 内容: '{}'", output.len(), output.trim());
        println!("错误输出长度: {}, 内容: '{}'", stderr.len(), stderr.trim());


        if exit_status != 0 && !stderr.trim().is_empty() {
            return Ok(format!("命令执行失败 (退出码: {}): {}", exit_status, stderr.trim()));
        }

        if output.trim().is_empty() && !stderr.trim().is_empty() {
            return Ok(stderr.trim().to_string());
        }

        if output.trim().is_empty() && stderr.trim().is_empty() {
            if exit_status == 0 {
                return Ok("命令执行成功".to_string());
            } else {
                return Ok(format!("命令执行完成 (退出码: {})", exit_status));
            }
        }

        Ok(output.trim().to_string())
    }
}
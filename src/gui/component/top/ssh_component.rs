
use uuid::Uuid;
use crate::gui::layout::Store;

#[derive(Default, Clone)]
pub struct SshComponent {
    pub ssh_username:String,
    pub ssh_password:String,
    pub ssh_port:String,
    pub ssh_host:String,
    pub id:Uuid,
    pub session_conn:Option<ssh2::Session>,
}


impl SshComponent {
    pub fn new() -> Self {
        SshComponent{
            ssh_username: "".to_string(),
            ssh_password: "".to_string(),
            ssh_port: "".to_string(),
            ssh_host: "".to_string(),
            id:Uuid::new_v4(),
            session_conn: None,
        }
    }

    pub fn from_edit_session( session: &SshComponent) -> Self {
        SshComponent {
            ssh_username: session.ssh_username.clone(),
            ssh_password: session.ssh_password.clone(),
            ssh_port: session.ssh_port.clone(),
            ssh_host: session.ssh_host.clone(),
            id: session.id,
            session_conn: session.session_conn.clone(),
        }
    }


    pub fn view(&mut self, ctx: &egui::Context, ui:&mut egui::Ui, store:&mut Store) -> egui::Response {
        ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200));
        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255));
        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::BLUE);


        ui.vertical(|ui| {
            ui.set_width(200.0);
            ui.label("左侧");
        });

        ui.separator();

        ui.vertical(|ui| {
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.set_width(80.0);
                    ui.label("用户名");
                });

                ui.add(
                    egui::TextEdit::singleline(&mut self.ssh_username)
                        .desired_width(f32::INFINITY)
                        .hint_text("请输入用户名")
                );
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.set_width(80.0);
                    ui.label("密码");
                });

                ui.add(
                    egui::TextEdit::singleline(&mut self.ssh_password)
                        .desired_width(f32::INFINITY)
                        .hint_text("请输入密码")
                );
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.set_width(80.0);
                    ui.label("端口号");
                });

                ui.add(
                    egui::TextEdit::singleline(&mut self.ssh_port)
                        .desired_width(f32::INFINITY)
                        .hint_text("请输入端口号")
                );


            });

            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.set_width(80.0);
                    ui.label("ip");
                });

                ui.add(
                    egui::TextEdit::singleline(&mut self.ssh_host)
                        .desired_width(f32::INFINITY)
                        .hint_text("请输入ip")
                );
            });

        });
        ui.response()

    }
}
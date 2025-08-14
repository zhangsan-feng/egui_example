use egui::InnerResponse;

#[derive(Default)]
pub struct SshComponent {
    pub ssh_username:String,
    pub ssh_password:String,
    pub ssh_port:String
}


impl SshComponent {
    pub fn new() -> Self {
        SshComponent{
            ssh_username: "".to_string(),
            ssh_password: "".to_string(),
            ssh_port: "".to_string(),
        }
    }

    pub fn view(&mut self, ctx: &egui::Context, ui:&mut egui::Ui) -> egui::Response {
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
                    egui::TextEdit::singleline(&mut self.ssh_username)
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

        });
        ui.response()

    }
}
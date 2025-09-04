use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use crate::font::font;
use crate::gui::component::center::center::Center;
use crate::gui::component::top::top_navigation::Navigation;
use crate::gui::component::left::left_session::LeftSession;
use crate::gui::component::top::ssh_component::SshComponent;



pub struct SessionContent {
    pub history_input: VecDeque<String>,
    pub current_input: String,
    pub prefix_input:String,
    // pub ssh_conn:
}

impl Default for Store {
    fn default() -> Self {
        let mut store = Store {
            session: Default::default(),
            active_session: Default::default(),
            session_content: Default::default(),
            default_session: Default::default(),
            show_new_session_window: Default::default(),
            show_color_picker: Default::default(),
            current_edit_session: Default::default(),
            is_editing_session: false,
        };

     

            let ssh_comp = SshComponent {
                ssh_username: "root".to_string(),
                ssh_password: "root".to_string(),
                ssh_port: "2222".to_string(),
                ssh_host: "192.168.2.101".to_string(),
                id: Uuid::new_v4(),
                session_conn: None,
            };
            
            store.session.push(ssh_comp);

        

        store
    }
}


pub struct Store{
    pub session:Vec<SshComponent>,
    pub active_session:Vec<SshComponent>,
    pub session_content:HashMap<Uuid,SessionContent>,

    pub default_session:Uuid,
    pub show_new_session_window:bool,
    pub show_color_picker:bool,
    pub current_edit_session:SshComponent,
    pub is_editing_session:bool,
}



pub struct ApplicationComponent {
    top_nav: Navigation,
    left_session: LeftSession,
    center:Center,
    left_panel_width: f32,
    pub store:Store,

}

impl ApplicationComponent {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        font::replace_fonts(&cc.egui_ctx);
        font::add_font(&cc.egui_ctx);
        Self {
            top_nav: Navigation::new(),
            left_session: LeftSession::new(),
            center:Center::new(),
            left_panel_width: 200.0,
            store:Store::default(),

        }
    }

}

impl eframe::App for ApplicationComponent {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(200));

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::Frame::default()
                .inner_margin(egui::Margin::same(5))
                .show(ui, |ui| {
                    self.top_nav.view(ctx, ui,&mut self.store)
                });
        });

        egui::SidePanel::left("left_panel").resizable(true).default_width(self.left_panel_width)
            .width_range(200.0..=400.0).show(ctx, |ui| {
            
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(5))
                    .show(ui, |ui| {
                        self.left_session.view(ctx, ui,&mut self.store);
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::default()
                // .inner_margin(egui::Margin::same(5))
                .show(ui, |ui| {
                    self.center.view(ctx, ui,&mut self.store);
                });
        });
    }
}
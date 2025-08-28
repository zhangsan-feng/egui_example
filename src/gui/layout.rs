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
            session: Vec::new(),
            active_session: Vec::new(),
            session_content: HashMap::new(),
            default_session: Uuid::new_v4(),
       
        };

     
        for i in 1..=200000 { 
            let ssh_comp = SshComponent {
                ssh_username: format!("user{}", i),
                ssh_password: format!("password{}", i),
                ssh_port: if i % 3 == 0 { "2222".to_string() } else { "22".to_string() },
                ssh_host: format!("服务器{} (192.168.1.{})", i, 100 + i),
                id: Uuid::new_v4(),
            };
            
            store.session.push(ssh_comp);
        }
        

        store
    }
}


pub struct Store{
    pub session:Vec<SshComponent>,
    pub active_session:Vec<SshComponent>,
    pub session_content:HashMap<Uuid,SessionContent>,
    pub default_session:Uuid,
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
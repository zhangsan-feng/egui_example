use egui::{Layout, Pos2, Sense, Vec2};
use crate::font::font;


#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum ApplicationPage {
    #[default]
    HomePage,
    TestPage
}
#[derive(Default)]
pub struct ApplicationComponent {
    current_page:ApplicationPage
}


impl ApplicationComponent {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        font::replace_fonts(&cc.egui_ctx);
        font::add_font(&cc.egui_ctx);
        Self {
            current_page:ApplicationPage::HomePage,
        }
    }


    pub fn home_page(&mut self, ui:&mut egui::Ui) -> egui::Response{
        ui.heading("home")

    }
    pub fn test_page(&mut self, ui:&mut egui::Ui) -> egui::Response{
        ui.heading("test")

    }

    pub fn main_home(&mut self, ui:&mut egui::Ui){

        egui::CentralPanel::default().show_inside(ui, |ui|{
            match self.current_page {
                ApplicationPage::HomePage =>{
                    self.home_page(ui)
                }
                ApplicationPage::TestPage =>{
                    self.test_page(ui)
                }
            }
        });

    }



}

impl eframe::App for ApplicationComponent {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").min_height(50.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("标题栏");
                });

            });
        });

        egui::SidePanel::left("left_panel").resizable(false).min_width(180.0).show(ctx,  |ui| {
            // ui.horizontal() 横
            // ui.vertical()   纵



            ui.vertical(|ui| {

                let widget_rect = egui::Rect::from_center_size(Pos2::new(100.0,100.0), Vec2::new(100.0,30.0));
                let rect = ui.allocate_exact_size(Vec2::new(100.0,30.0).into(),Sense::hover());

                ui.child_ui( rect.0, Layout::centered_and_justified(egui::Direction::BottomUp), None )
                    .selectable_value(&mut self.current_page, ApplicationPage::HomePage, "HomePage");


                let rect = ui.allocate_exact_size(Vec2::new(100.0,30.0).into(),Sense::hover());
                ui.child_ui( rect.0, Layout::centered_and_justified(egui::Direction::TopDown), None )
                    .selectable_value(&mut self.current_page, ApplicationPage::TestPage, "TestPage");

            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // egui::Window::new("").vscroll(true).show(ctx,|ui|{ctx.settings_ui(ui)});
            self.main_home(ui)
        });
    }

}

use crate::font::font;
use super::navigation::Navigation;


#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ApplicationPage {  // 添加 pub 关键字使其公有
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


        egui::TopBottomPanel::top("top_panel")
        .min_height(50.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("标题栏");
                });
            });
        });

        egui::SidePanel::left("left_panel")
        .resizable(false)
        .min_width(180.0)

        .show(ctx, |ui| {
            Navigation::render(ui, &mut self.current_page);
        });

        egui::CentralPanel::default()
        .show(ctx, |ui| {
            self.main_home(ui)
        });
    }
}

use egui_extras::{Column, TableBuilder};

pub struct ProcessManager {}

impl ProcessManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            let column_width = ui.available_width() / 5.0 - 10.0;
            TableBuilder::new(ui)
                .column(Column::initial(column_width))
                .column(Column::initial(column_width))
                .column(Column::initial(column_width))
                .column(Column::initial(column_width))
                .column(Column::initial(column_width))
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.strong("ID"); });
                    header.col(|ui| { ui.strong("Name"); });
                    header.col(|ui| { ui.strong("Value"); });
                    header.col(|ui| { ui.strong("Action"); });
                    header.col(|ui| { ui.strong("Action"); });
                })
                .body(|mut body| {
                    for i in 1..100 {
                        body.row(10.0, |mut row| {
                            row.col(|ui| { ui.label(i.to_string()); });
                            row.col(|ui| { ui.label("Item B"); });
                            row.col(|ui| { ui.label("200"); });
                            row.col(|ui| {
                                if ui.button("Edit").clicked() {
                                    println!("Edit button clicked for row {}", i);
                                }
                            });
                            row.col(|ui| {
                                if ui.button("Kill").clicked() {
                                    println!("Edit button clicked for row {}", i);
                                }
                            });
                        });
                    }
                });
        }).response
    }

}
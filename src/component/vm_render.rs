use egui::{ScrollArea, Ui};

#[derive(Default)]
pub struct VirtualizedTextRenderer {
    lines: Vec<String>,
    line_height: f32,
}

impl VirtualizedTextRenderer {
    pub fn new(content: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self {
            lines,
            line_height: 20.0, // 每行的高度
        }
    }

    pub fn view(&mut self, ui: &mut Ui) {
        let total_lines = self.lines.len();
        let available_height = ui.available_height().min(400.0);

        ScrollArea::vertical()
            .max_height(available_height)
            .auto_shrink([false, true])
            .show_rows(
                ui,
                self.line_height,
                total_lines,
                |ui, row_range| {
                    for row in row_range {
                        if row < self.lines.len() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{:4}: {}", row + 1, &self.lines[row]));
                            });
                        }
                    }
                }
            );
    }
}
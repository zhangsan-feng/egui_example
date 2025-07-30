/*

列布局
ui.vertical(|ui| {})

行布局
ui.horizontal(|ui| {})


滑动条
egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {})



创建窗口
egui::Window::new("123")
    .title_bar(false)
    .fixed_size([800.0, 600.0])
    .resizable(false)
    .movable(true)
    .collapsible(false)

    .anchor(egui::Align2::CENTER_CENTER, [0.0, -50.0])
    .default_size([400.0, 300.0])
    .resizable(false)
    .movable(false)
    .frame(egui::Frame {
        fill: egui::Color32::WHITE,
        stroke: egui::Stroke::NONE,
        inner_margin: egui::Margin::same(15),
        outer_margin: egui::Margin::same(15),
        shadow: egui::epaint::Shadow::NONE,
        corner_radius: egui::CornerRadius::same(5),
    })
    .show(ctx, |ui| {})



ui.allocate_ui_with_layout(
    egui::Vec2::new(200.0, ui.available_height()),
    egui::Layout::top_down(egui::Align::LEFT),
    |ui| {}
);

ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {})

*/
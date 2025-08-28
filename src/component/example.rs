use egui::{Align, ScrollArea, Ui, Vec2};

#[derive(Default)]
pub struct ExampleComponent {
    lines: Vec<String>,
    line_height: f32,
}

impl ExampleComponent {
    pub fn view(&mut self, ui: &mut Ui) {
        // 修改背景样式
        ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200));
        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255));
        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::BLUE);
        /*
        - `bg_fill` - 背景填充色
        - - 边框（颜色和宽度） `bg_stroke`
        - - 前景色（文本颜色） `fg_stroke`
        - - 未激活状态 `inactive`
        - `hovered` - 鼠标悬停状态
        - `active` - 激活状态（获得焦点时）
        
        - `d.insert_temp(id, value)` - 插入临时数据
        - `d.get_temp::<T>(id)` - 获取临时数据
        - `d.insert_persisted(id, value)` - 插入持久数据
        - `d.get_persisted::<T>(id)` - 获取持久数据
        
         */

        // 布局
        ui.allocate_ui_with_layout(
            egui::Vec2::new(200.0, ui.available_height()),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {}
        );

        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {});
        
        // 设置宽高
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());


        // 列布局
        ui.vertical(|ui| {});

        // 行布局
        ui.horizontal(|ui| {});


        // 滑动条
        egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {});

        // 添加分割线
        ui.separator();

        // 添加间距
        ui.spacing_mut().item_spacing.y = 20.0;  // 垂直间距
        ui.spacing_mut().item_spacing.x = 20.0;  // 水平间距
        ui.spacing_mut().button_padding = egui::vec2(10.0, 6.0);  // 按钮内边距
        ui.spacing_mut().indent = 25.0;  // 缩进距离
        ui.add_space(5.0);
    }
        
    

}
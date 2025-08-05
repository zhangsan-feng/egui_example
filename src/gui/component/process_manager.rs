use egui_extras::{Column, TableBuilder};
use std::collections::HashMap;

pub struct ProcessManager {
    pub data: Vec<ProcessItem>,
    // 添加临时编辑状态存储
    pub edit_states: HashMap<usize, EditState>,
}

#[derive(Clone)]
pub struct ProcessItem {
    pub id: usize,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Default)]
pub struct EditState {
    pub name: String,
    pub value: String,
    pub is_editing: bool,
}

impl ProcessManager {
    pub fn new() -> Self {
        let data = (1..100).map(|i| ProcessItem {
            id: i,
            name: format!("Process {}", i),
            value: "200".to_string(),
        }).collect();
        
        Self { 
            data,
            edit_states: HashMap::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut items_to_remove = Vec::new();
        let mut items_to_save = Vec::new();
        
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
                    header.col(|ui| { ui.strong("Edit"); });
                    header.col(|ui| { ui.strong("Action"); });
                })
                .body(|mut body| {
                    for (index, item) in self.data.iter().enumerate() {
                        body.row(10.0, |mut row| {
                            // 获取或创建编辑状态
                            let edit_state = self.edit_states.entry(item.id).or_insert_with(|| {
                                EditState {
                                    name: item.name.clone(),
                                    value: item.value.clone(),
                                    is_editing: false,
                                }
                            });

                            // 显示 ID
                            row.col(|ui| { 
                                ui.label(item.id.to_string()); 
                            });
                            
                            // 名称列 - 根据编辑状态显示不同控件
                            row.col(|ui| {
                                if edit_state.is_editing {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut edit_state.name)
                                            .id(egui::Id::new(format!("name_{}", item.id)))
                                    );
                                } else {
                                    ui.label(&item.name);
                                }
                            });
                            
                            // 值列 - 根据编辑状态显示不同控件
                            row.col(|ui| {
                                if edit_state.is_editing {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut edit_state.value)
                                            .id(egui::Id::new(format!("value_{}", item.id)))
                                    );
                                } else {
                                    ui.label(&item.value);
                                }
                            });
                            
                            // 编辑/保存按钮
                            row.col(|ui| {
                                if edit_state.is_editing {
                                    ui.horizontal(|ui| {
                                        if ui.small_button("保存").clicked() {
                                            items_to_save.push(item.id);
                                        }
                                        if ui.small_button("取消").clicked() {
                                            // 恢复原始值
                                            edit_state.name = item.name.clone();
                                            edit_state.value = item.value.clone();
                                            edit_state.is_editing = false;
                                        }
                                    });
                                } else {
                                    if ui.button("编辑").clicked() {
                                        edit_state.is_editing = true;
                                        // 初始化编辑值
                                        edit_state.name = item.name.clone();
                                        edit_state.value = item.value.clone();
                                    }
                                }
                            });
                            
                            // 删除按钮
                            row.col(|ui| {
                                if ui.button("删除").clicked() {
                                    items_to_remove.push(index);
                                }
                            });
                        });
                    }
                });
        });

        // 处理保存操作
        for item_id in items_to_save {
            if let Some(edit_state) = self.edit_states.get(&item_id) {
                if let Some(item) = self.data.iter_mut().find(|item| item.id == item_id) {
                    item.name = edit_state.name.clone();
                    item.value = edit_state.value.clone();
                    println!("保存第 {} 行的更改: name={}, value={}", 
                           item.id, item.name, item.value);
                }
            }
            // 退出编辑模式
            if let Some(edit_state) = self.edit_states.get_mut(&item_id) {
                edit_state.is_editing = false;
            }
        }
        
        // 处理删除操作（从后往前删除）
        for &index in items_to_remove.iter().rev() {
            let item_id = self.data[index].id;
            self.data.remove(index);
            self.edit_states.remove(&item_id);
            println!("删除第 {} 行", item_id);
        }

        ui.response()
    }

    pub fn add_item(&mut self, name: String, value: String) {
        let new_id = self.data.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        self.data.push(ProcessItem {
            id: new_id,
            name,
            value,
        });
    }
}
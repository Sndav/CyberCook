use std::default::Default;

use eframe::{
    egui,
    egui::{Align, Color32, DragAndDrop, Frame, Id, LayerId, Layout, Order, Sense, TextEdit},
    emath,
};
use log::debug;

use crate::{
    modules::{
        crypto::aes::AESEncrypt,
        encoding::base64::{Base64Decoder, Base64Encoder},
        Module,
    },
    views::splitter::{Splitter, SplitterAxis},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    col: usize,
    row: usize,
}

pub struct CyberCook {
    pub available_module: Vec<Box<dyn Module>>,
    pub selected_module: Vec<Box<dyn Module>>,
    pub input: String,
}

impl eframe::App for CyberCook {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_header(ctx);
            self.ui_module_list(ctx);
            self.ui_input_output(ctx, ui);
            self.ui_selected_modules(ctx);
        });
    }
}

impl CyberCook {
    pub fn new(available_module: Vec<Box<dyn Module>>) -> Self {
        Self {
            available_module,
            selected_module: vec![],
            input: "".to_string(),
        }
    }

    fn ui_header(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            // header
            ui.horizontal(|ui| {
                // banner
                ui.label("CyberCook");
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        println!("Open");
                    }
                    if ui.button("Save").clicked() {
                        println!("Save");
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                });
            });
        });
    }

    fn ui_module_list(&mut self, ctx: &egui::Context) {
        let frame = Frame::default().inner_margin(1.0);
        egui::SidePanel::left("module_list")
            .resizable(true)
            .show(ctx, |ui| {
                // margin top
                let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (index, module) in self.available_module.iter().enumerate() {
                            let drag_id = Id::new(module.id());

                            let item_location = Location { col: 0, row: index };
                            let response = ui
                                .dnd_drag_source(drag_id, item_location, |ui| {
                                    module.render_list(ui, index);
                                })
                                .response;

                            if response.hovered() {
                                if let Some(dragged_payload) =
                                    response.dnd_release_payload::<Location>()
                                {
                                    self.selected_module.remove(dragged_payload.row);
                                }
                            }
                        }
                    });

                    ui.allocate_space(ui.available_size());
                });
                if let Some(dragged_payload) = dropped_payload {
                    self.selected_module.remove(dragged_payload.row);
                }
            });
    }

    fn ui_selected_modules(&mut self, ctx: &egui::Context) {
        let frame = Frame::default().inner_margin(2.0); // No margin around the

        // CentralPanel width is 200px
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut dropped_module = None;
            let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, module) in self.selected_module.iter_mut().enumerate() {
                        let drag_id = Id::new(module.id());

                        let item_location = Location { col: 1, row: index };

                        let is_being_dragged = ctx.is_being_dragged(drag_id);

                        let mut allow_check_for_drag = false;
                        ui.input(|input| {
                            if let Some(press_origin) = input.pointer.press_origin() {
                                if let Some(latest_pos) = input.pointer.latest_pos() {
                                    let distance = press_origin.distance(latest_pos);
                                    if distance > 6.0 {
                                        allow_check_for_drag = true;
                                    }
                                }
                            }
                        });

                        let can_drag = is_being_dragged && allow_check_for_drag;

                        let response = if !can_drag {
                            ui.scope(|ui| {
                                module.render(ui, index);
                            })
                            .response
                        } else {
                            DragAndDrop::set_payload(ctx, item_location);
                            let layer_id = LayerId::new(Order::Tooltip, drag_id);

                            let response = ui
                                .with_layer_id(layer_id, |ui| module.render(ui, index))
                                .response;

                            if let Some(pointer_pos) = ctx.pointer_interact_pos() {
                                let delta = pointer_pos - response.rect.center();
                                ctx.transform_layer_shapes(
                                    layer_id,
                                    emath::TSTransform::from_translation(delta),
                                );
                            }

                            response
                        };

                        if let (Some(pointer), Some(hovered_payload)) = (
                            ui.input(|i| i.pointer.interact_pos()),
                            response.dnd_hover_payload::<Location>(),
                        ) {
                            debug!("Pointer: {:?}, Hovered: {:?}", pointer, hovered_payload);
                            let rect = response.rect;

                            // Preview insertion:
                            let stroke = egui::Stroke::new(1.0, Color32::WHITE);
                            let insert_index = if *hovered_payload == item_location {
                                // We are dragged onto ourselves
                                ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                                index
                            } else if pointer.y < rect.center().y {
                                // Above us
                                ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                index
                            } else {
                                // Below us
                                ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                index + 1
                            };

                            if let Some(dragged_payload) =
                                response.dnd_release_payload::<Location>()
                            {
                                // The user dropped onto this item.
                                debug!("Dropped {:?} onto {:?}", dragged_payload, insert_index);
                                dropped_module = Some((dragged_payload, insert_index));
                            }
                        }
                    }
                });
                // set height to 100%
                ui.allocate_space(ui.available_size());
            });

            if let Some((dragged_payload, insert_index)) = dropped_module {
                if dragged_payload.col == 1 {
                    // 上下拖动
                    let module = self.selected_module.remove(dragged_payload.row);
                    if dragged_payload.row < insert_index {
                        self.selected_module.insert(insert_index - 1, module);
                    } else {
                        self.selected_module.insert(insert_index, module);
                    }
                } else if let Some(module) = self.available_module.get(dragged_payload.row) {
                    self.selected_module
                        .insert(insert_index, module.clone_box());
                }
            } else if let Some(dragged_payload) = dropped_payload {
                if dragged_payload.col == 0 {
                    // 左右拖动
                    if let Some(module) = self.available_module.get(dragged_payload.row) {
                        self.selected_module.push(module.clone_box());
                    }
                } else {
                    // 上下拖动
                    let module = self.selected_module.remove(dragged_payload.row);
                    self.selected_module.push(module);
                }
            }
        });
    }

    fn ui_input_output(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let Self { input, .. } = self;
        let font_size = ctx
            .style()
            .text_styles
            .get(&egui::TextStyle::Body)
            .unwrap()
            .size;

        egui::SidePanel::right("input_output")
            .min_width(ui.available_width() / 2.0)
            .resizable(true)
            .show_separator_line(false)
            .show(ctx, |ui| {
                Splitter::new("input_output", SplitterAxis::Vertical).show(ui, |up_ui, down_ui| {
                    // Input 区域
                    up_ui.vertical(|ui| {
                        ui.heading("Input");
                        egui::ScrollArea::vertical()
                            .id_salt("input")
                            .show(ui, |ui| {
                                ui.add(
                                    TextEdit::multiline(input)
                                        .desired_width(ui.available_width())
                                        .desired_rows((ui.available_height() / font_size) as _)
                                        .hint_text("Input here"),
                                );
                            })
                    });

                    let mut output = input.as_bytes().to_vec();
                    for module in self.selected_module.iter() {
                        match module.process(&output) {
                            Ok(new_output) => {
                                output = new_output;
                            }
                            Err(e) => {
                                output = format!("{}", e).as_bytes().to_vec();
                                break;
                            }
                        }
                    }

                    let mut output = String::from_utf8_lossy(&output).to_string();

                    // Output 区域
                    down_ui.vertical(|ui| {
                        ui.heading("Output");
                        egui::ScrollArea::vertical()
                            .id_salt("output")
                            .show(ui, |ui| {
                                ui.add(
                                    TextEdit::multiline(&mut output)
                                        .desired_width(ui.available_width())
                                        .desired_rows((ui.available_height() / font_size) as _)
                                        .hint_text("Output here"),
                                );
                            });
                    });
                });
            });
    }
}

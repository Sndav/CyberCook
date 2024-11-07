use eframe::egui::{Color32, Frame, Id, Response, Sense, Ui};

pub mod crypto;
pub mod encoding;
mod input;

pub trait Module {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn id(&self) -> &str;
    fn process(&self, input: &[u8]) -> anyhow::Result<Vec<u8>>;

    fn render_inner(&mut self, ui: &mut Ui) {}

    fn render(&mut self, ui: &mut Ui, index: usize) {
        let background_color = if ui.visuals().dark_mode {
            if index % 2 == 0 {
                Color32::from_rgb(40, 60, 60)
            } else {
                Color32::from_rgb(30, 50, 50)
            }
        } else if index % 2 == 0 {
            Color32::from_rgb(240, 220, 220)
        } else {
            Color32::from_rgb(230, 210, 210)
        };

        Frame::none().fill(background_color).show(ui, |ui| {
            ui.group(|ui| {
                ui.set_min_height(32.0);
                ui.set_min_width(ui.available_width());
                ui.vertical(|ui| {
                    // set drag detection area
                    let resp = ui
                        .horizontal(|ui| {
                            ui.heading(self.name());
                            ui.label(self.description());
                        })
                        .response;
                    ui.interact(resp.rect, Id::new(self.id()), Sense::drag());
                    self.render_inner(ui);
                });
            });
        });
    }
    fn render_list(&self, ui: &mut Ui, index: usize) -> Response {
        // 使用交替背景颜色
        // light theme: 240, 220
        // dark theme: 40, 60

        let background_color = if ui.visuals().dark_mode {
            if index % 2 == 0 {
                Color32::from_rgb(40, 60, 60)
            } else {
                Color32::from_rgb(30, 50, 50)
            }
        } else if index % 2 == 0 {
            Color32::from_rgb(240, 220, 220)
        } else {
            Color32::from_rgb(230, 210, 210)
        };

        let frame = Frame::none().fill(background_color).show(ui, |ui| {
            ui.group(|ui| {
                ui.set_min_height(32.0);
                ui.set_min_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.heading(self.name());
                    ui.label(self.description());
                    //
                });
            });
        });

        frame.response
    }

    fn clone_box(&self) -> Box<dyn Module>;
}

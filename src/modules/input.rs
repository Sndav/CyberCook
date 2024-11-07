use eframe::{egui, egui::Id};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

#[derive(Default, EnumIter, Display, PartialEq, Eq, Clone, Copy)]
pub enum InputType {
    #[default]
    #[strum(to_string = "Text")]
    Text = 0,
    #[strum(to_string = "Hex")]
    Hex = 1,
    #[strum(to_string = "Base64")]
    Base64 = 2,
}

pub struct Argument {
    id: String,
    value: String,
    input_type: InputType,
}

impl Default for Argument {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            value: String::new(),
            input_type: InputType::Text,
        }
    }
}

impl Argument {
    pub fn try_to_vec(&self) -> anyhow::Result<Vec<u8>> {
        match self.input_type {
            InputType::Text => Ok(self.value.as_bytes().to_vec()),
            InputType::Hex => Ok(hex::decode(&self.value)?),
            InputType::Base64 => Ok(base64::decode(&self.value)?),
        }
    }

    pub fn try_to_str(&self) -> anyhow::Result<String> {
        let bytes = self.try_to_vec()?;
        Ok(String::from_utf8(bytes)?)
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt(Id::new(&self.id))
                .selected_text(self.input_type.to_string())
                .show_ui(ui, |ui| {
                    for input_type in InputType::iter() {
                        ui.selectable_value(
                            &mut self.input_type,
                            input_type,
                            input_type.to_string(),
                        );
                    }
                });
            ui.text_edit_singleline(&mut self.value);
        });
    }
}

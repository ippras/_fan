use crate::r#const::SETTINGS;
use egui::{ComboBox, Grid, Id, Sense, Ui};
use egui_l10n::ContextExt;
use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) left_panel: bool,
    pub(crate) open: bool,
    pub(crate) reactive: bool,
    pub(crate) reset_state: bool,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Self {
            left_panel: true,
            open: false,
            reactive: true,
            reset_state: false,
        }
    }
}

impl Settings {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        let id_salt = Id::new(SETTINGS);
        Grid::new(id_salt).show(ui, |ui| {
            // Language
            ui.label(ui.localize("Language"));
            let mut current_value = ui.language_identifier();
            ComboBox::from_id_salt(id_salt.with("Language"))
                .selected_text(current_value.to_string())
                .show_ui(ui, |ui| {
                    let mut response = ui.allocate_response(Default::default(), Sense::click());
                    for selected_value in ui.language_identifiers() {
                        let text = selected_value.to_string();
                        response |= ui.selectable_value(&mut current_value, selected_value, text);
                    }
                    if response.changed() {
                        ui.set_language_identifier(current_value);
                    }
                });
            ui.end_row();
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

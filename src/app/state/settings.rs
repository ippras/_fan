use egui::{Response, Ui};
use serde::{Deserialize, Serialize};

/// Settings state
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) open: bool,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Self { open: false }
    }

    pub(crate) fn ui(&mut self, ui: &mut Ui) -> Response {
        let response = ui.label("text");
        response
    }
}

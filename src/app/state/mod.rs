pub(super) use self::{about::About, settings::Settings, windows::Windows};

use egui::{Id, Ui};
use serde::{Deserialize, Serialize};

/// Settings window
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct State {
    pub(crate) settings: Settings,
    pub(crate) windows: Windows,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            settings: Settings::new(),
            windows: Windows::new(),
        }
    }
}

impl State {
    pub fn load(ui: &Ui, id: Id) -> Self {
        ui.data_mut(|data| {
            data.get_persisted_mut_or_insert_with(id, || Self::new())
                .clone()
        })
    }

    pub fn remove(self, ui: &Ui, id: Id) {
        ui.data_mut(|data| {
            data.remove::<Self>(id);
        });
    }

    pub fn store(self, ui: &Ui, id: Id) {
        ui.data_mut(|data| {
            data.insert_persisted(id, self);
        });
    }
}

mod about;
mod settings;
mod windows;

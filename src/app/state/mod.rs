pub(super) use self::{about::About, settings::Settings, windows::Windows};

use egui::{Context, Id};
use serde::{Deserialize, Serialize};

/// Settings window
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct State {
    pub(crate) about: About,
    pub(crate) settings: Settings,
    pub(crate) windows: Windows,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            about: About,
            settings: Settings::new(),
            windows: Windows::new(),
        }
    }
}

impl State {
    pub fn load(ctx: &Context, id: Id) -> Self {
        ctx.data_mut(|data| {
            data.get_persisted_mut_or_insert_with(id, || Self::new())
                .clone()
        })
    }

    pub fn remove(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| {
            data.remove::<Self>(id);
        });
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| {
            data.insert_persisted(id, self);
        });
    }
}

mod about;
mod settings;
mod windows;

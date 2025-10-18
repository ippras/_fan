use serde::{Deserialize, Serialize};

/// Windows state
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Windows {
    pub(crate) open_about: bool,
    pub(crate) open_edit: bool,
    pub(crate) open_settings: bool,
}

impl Windows {
    pub(crate) fn new() -> Self {
        Self {
            open_about: false,
            open_edit: false,
            open_settings: false,
        }
    }
}

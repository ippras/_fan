use egui::{
    CentralPanel, Color32, Grid, Id, Label, MenuBar, Popup, PopupCloseBehavior, RichText,
    ScrollArea, Tooltip, TopBottomPanel, Ui, Widget,
};
use egui_dnd::dnd;
use egui_l20n::{ResponseExt, UiExt as _};
use egui_phosphor::regular::{CHECK, DOTS_SIX_VERTICAL, PENCIL_LINE, TRASH};
use polars::prelude::PolarsResult;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const ITEMS: [&str; 165] = [
    "c2",
    "c3",
    "c3a2",
    "c3e2",
    "c4",
    "c4c2",
    "c4t2",
    "c5",
    "c6",
    "c6t3",
    "c6t2t4",
    "c7",
    "c8",
    "c9",
    "c10",
    "c10c4",
    "c10e9",
    "c10c2c4",
    "c10t2c4",
    "c10c2a4a6",
    "c10c2a4a6t8",
    "c10e2a4a6a8",
    "c10t2a4a6t8",
    "c11",
    "c11e10",
    "c11c3a5a7a10",
    "c11c3a5a7e9e10",
    "c12",
    "c12t4",
    "c12c5",
    "c12c9",
    "c12c3c5c7c9e11",
    "c13",
    "c13c11",
    "c13c3c5a7a9a11",
    "c13t3c5e7e8a10a12",
    "c14",
    "c14c4",
    "c14c5",
    "c14c9",
    "c14t9",
    "c14t3c5",
    "c14c5c8",
    "c14t2t4a8a10",
    "c16",
    "c16c11",
    "c16t11",
    "c16t2",
    "c16c6",
    "c16a7",
    "c16c7",
    "c16c9",
    "c16t9",
    "c16c6c10c14",
    "c16c7c10c13",
    "c17",
    "c17c8",
    "c17a8t10",
    "c17c8c11",
    "c17c8c11c14",
    "c18",
    "c18c10",
    "c18c11",
    "c18t11",
    "c18t5",
    "c18a6",
    "c18c6",
    "c18t6",
    "c18a9",
    "c18c9",
    "c18t9",
    "c18t10t12",
    "c18t11c15",
    "c18c5c11",
    "c18c5c8",
    "c18c5c9",
    "c18e5e6",
    "c18a6e17",
    "c18c6c11",
    "c18a8c10",
    "c18a8e17",
    "c18a9t11",
    "c18c9c11",
    "c18c9t11",
    "c18c9a12",
    "c18c9c12",
    "c18t9t11",
    "c18t9t12",
    "c18t10t12t14",
    "c18t3c9c12",
    "c18c5c9c12",
    "c18r5e6c16",
    "c18s5e6c16",
    "c18t5c9c12",
    "c18c6c9c12",
    "c18c8c10c12",
    "c18c8c10t12",
    "c18c8t10c12",
    "c18a9a11c13",
    "c18a9a11t13",
    "c18a9a11e17",
    "c18c9t11c13",
    "c18c9t11t13",
    "c18c9t11c15",
    "c18c9a12c14",
    "c18c9c12c15",
    "c18c9t13c15",
    "c18t9t11c13",
    "c18t9t11t13",
    "c18t9t12t15",
    "c18c5c9c12c15",
    "c18a6c9c12c15",
    "c18c6c9c12c15",
    "c18c8c10c12t14",
    "c18c9t11t13c15",
    "c18t9t11t13t15",
    "c20",
    "c20c11",
    "c20c13",
    "c20c9",
    "c20t9",
    "c20c11c14",
    "c20c5c11",
    "c20c7c11",
    "c20e7e8",
    "c20c11c14c17",
    "c20c5c11c14",
    "c20c5c8c11",
    "c20c7c11c14",
    "c20c7c11t14",
    "c20c8c11c14",
    "c20c4c8c12c15",
    "c20c5c11c14c17",
    "c20c5c8c11c14",
    "c20c5c8t10t12c14",
    "c20c5c8c11c14c17",
    "c22",
    "c22c11",
    "c22t11",
    "c22a13",
    "c22c13",
    "c22t13",
    "c22c5c13c16",
    "c22c7c10c13",
    "c22c7c10c13c16",
    "c22c4c8c12c15c19",
    "c22c7c10c13c16c19",
    "c22c4c7c10c13c16c19",
    "c24",
    "c24c15",
    "c24t15",
    "c24c6c9c12c15c18c21",
    "c25",
    "c26",
    "c26c17",
    "c26c11c14c17c20c23",
    "c26e8e11e14e17e20e23",
    "c27",
    "c28",
    "c30",
    "c30c21",
    "c32",
    "c33",
    "c34",
    "c35",
];

/// Data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub items: Vec<String>,
    pub current: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            items: ITEMS.iter().map(|item| item.to_string()).collect(),
            current: String::new(),
        }
    }

    // pub fn add(&mut self, frame: String) {
    //     if !self.items.contains(&frame) {
    //         self.items.push(frame);
    //     }
    // }
}

impl Data {
    pub(super) fn show(&mut self, ui: &mut Ui) {
        // TopBottomPanel::top(ui.auto_id_with("LeftPane").with("TopPane")).show_inside(ui, |ui| {
        //     MenuBar::new().ui(ui, |ui| {
        //         self.top(ui);
        //     });
        // });
        // CentralPanel::default().show_inside(ui, |ui| {
        //     ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
        //         self.central(ui);
        //     });
        // });
    }

    pub(super) fn top(&mut self, ui: &mut Ui) {
        ui.heading(ui.localize("loaded-files"))
            .on_hover_localized("loaded-files.hover");
        ui.separator();
        // Toggle
        if ui
            .button(RichText::new(CHECK).heading())
            .on_hover_localized("toggle-all")
            .on_hover_localized("toggle-all.hover")
            .clicked()
        {
            if self.current.is_empty() {
                self.current = self.items.iter().cloned().collect();
            } else {
                self.current.clear();
            }
        }
        ui.separator();
        let enabled = !self.current.is_empty();
        // // Delete
        // ui.add_enabled_ui(enabled, |ui| {
        //     if ui
        //         .button(RichText::new(TRASH).heading())
        //         .on_hover_localized("DeleteSelected.hover")
        //         .clicked()
        //     {
        //         self.items.retain(|frame| !self.selected.remove(frame));
        //     }
        // });
        ui.separator();
    }

    pub(super) fn left(&mut self, ui: &mut Ui) {
        dnd(ui, ui.next_auto_id()).show_vec(&mut self.items, |ui, selected, handle, _state| {
            ui.horizontal(|ui| {
                handle.ui(ui, |ui| {
                    let _ = ui.label(DOTS_SIX_VERTICAL);
                });
                // Label
                let mut response = ui.selectable_label(self.current == *selected, &*selected);
                if response.clicked() {
                    self.current = if self.current != *selected {
                        selected.clone()
                    } else {
                        String::new()
                    };
                    response.mark_changed();
                }
            });
        });
    }

    pub(super) fn central(&mut self, ui: &mut Ui) {
        // ui.visuals_mut().widgets.inactive.bg_fill = Color32::TRANSPARENT;
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            if let Some(abbreviation) = ui.try_localize(&format!("{}.abbreviation", self.current)) {
                ui.heading(ui.localize("Abbreviation"));
                ui.label(abbreviation);
                ui.end_row();
            }
            if let Some(mut common) = ui.try_localize(&format!("{}.common", self.current)) {
                ui.heading(ui.localize("Common"));
                ui.label(&common);
                let response = ui.button(PENCIL_LINE);
                Popup::from_toggle_button_response(&response)
                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                    .show(
                        move |ui| {
                            if ui.text_edit_singleline(&mut common).changed() {}
                        },
                    );
                ui.end_row();
            }
            if let Some(mut synonyms) = ui.try_localize(&format!("{}.synonyms", self.current)) {
                ui.heading(ui.localize("Synonyms"));
                ui.label(&synonyms);
                let response = ui.button(PENCIL_LINE);
                Popup::from_toggle_button_response(&response)
                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                    .show(
                        move |ui| {
                            if ui.text_edit_singleline(&mut synonyms).changed() {}
                        },
                    );
                ui.end_row();
            }
            if let Some(mut systematic) = ui.try_localize(&format!("{}.systematic", self.current)) {
                ui.heading(ui.localize("Systematic"));
                ui.label(&systematic);
                let response = ui.button(PENCIL_LINE);
                Popup::from_toggle_button_response(&response)
                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                    .show(
                        move |ui| {
                            if ui.text_edit_singleline(&mut systematic).changed() {}
                        },
                    );
                ui.end_row();
            }
        });
    }
}

use self::state::{About, Settings, State, Windows};
use crate::{
    app::data::Data,
    r#const::{BOTTOM_PANEL, LEFT_PANEL, STATE, TOP_PANEL},
    localization::ContextExt as _,
};
use anyhow::Result;
use eframe::{APP_KEY, CreationContext, Storage, get_value, set_value};
use egui::{
    Align, CentralPanel, FontDefinitions, Frame, Id, Label, Layout, MenuBar, Panel, RichText,
    ScrollArea, Sides, Ui, Widget, Window, warn_if_debug_build,
};
use egui_ext::LightDarkButton;
use egui_l10n::{UiExt as _, ui::locale_button::LocaleButton};
use egui_phosphor::{
    Variant, add_to_fonts,
    regular::{
        ARROWS_CLOCKWISE, FILE, FLOPPY_DISK, GEAR, INFO, PENCIL, SIDEBAR_SIMPLE,
        SLIDERS_HORIZONTAL, TRASH,
    },
};
use polars::prelude::*;
use ron::ser::{PrettyConfig, to_string_pretty, to_writer};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::Cursor, sync::Arc};
use tracing::instrument;
use widgets::buttons::{LeftPanelButton, ReactiveButton, ResetButton};
use fatty_acid_names;

const ICON_SIZE: f32 = 32.0;
const ID_SOURCE: &str = "FAN";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Data
    data: Data,
}

impl Default for App {
    fn default() -> Self {
        Self { data: Data::new() }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();
        add_to_fonts(&mut fonts, Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_localizations();
        // return Default::default();
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Self::load(cc).unwrap_or_default()
    }

    fn load(cc: &CreationContext) -> Option<Self> {
        let storage = cc.storage?;
        let value = get_value(storage, APP_KEY)?;
        Some(value)
    }
}

// Panels
impl App {
    fn panels(&mut self, ui: &mut Ui, state: &mut State) {
        self.top_panel(ui, state);
        self.bottom_panel(ui);
        self.left_panel(ui, state);
        self.central_panel(ui);
    }

    // Bottom panel
    fn bottom_panel(&mut self, ui: &mut Ui) {
        Panel::bottom(BOTTOM_PANEL).show_inside(ui, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                Sides::new().show(
                    ui,
                    |_| {},
                    |ui| {
                        warn_if_debug_build(ui);
                        ui.label(RichText::new(env!("CARGO_PKG_VERSION")).small());
                        ui.separator();
                    },
                );
            });
        });
    }

    // Central panel
    fn central_panel(&mut self, ui: &mut Ui) {
        CentralPanel::default()
            .frame(Frame::central_panel(&ui.style()))
            .show_inside(ui, |ui| {
                self.data.central(ui);
            });
    }

    // Left panel
    fn left_panel(&mut self, ui: &mut Ui, state: &mut State) {
        Panel::left(LEFT_PANEL)
            .resizable(true)
            .show_animated_inside(ui, state.settings.left_panel, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.data.left(ui);
                });
            });
    }

    // Top panel
    fn top_panel(&mut self, ui: &mut Ui, state: &mut State) {
        Panel::top(TOP_PANEL).show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    // Left panel
                    LeftPanelButton::builder()
                        .selected(&mut state.settings.left_panel)
                        .size(ICON_SIZE)
                        .build()
                        .ui(ui);
                    ui.separator();
                    ReactiveButton::builder()
                        .selected(&mut state.settings.reactive)
                        .size(ICON_SIZE)
                        .build()
                        .ui(ui);
                    ui.separator();
                    // Light/Dark
                    ui.light_dark_button(ICON_SIZE);
                    ui.separator();
                    // Reset
                    ResetButton::builder()
                        .selected(&mut state.settings.reset_state)
                        .size(ICON_SIZE)
                        .build()
                        .ui(ui);
                    ui.separator();

                    // // Left panel
                    // ui.toggle_value(
                    //     &mut state.settings.left_panel,
                    //     RichText::new(SIDEBAR_SIMPLE).size(ICON_SIZE),
                    // )
                    // .on_hover_ui(|ui| {
                    //     ui.set_max_width(ui.spacing().tooltip_width);
                    //     ui.label(ui.localize(LEFT_PANEL));
                    // });
                    // ui.separator();
                    // // Light/Dark
                    // ui.light_dark_button(ICON_SIZE);
                    // ui.separator();
                    // // Reset
                    // if ui
                    //     .button(RichText::new(ARROWS_CLOCKWISE).size(ICON_SIZE))
                    //     .on_hover_ui(|ui| {
                    //         ui.set_max_width(ui.spacing().tooltip_width);
                    //         ui.label(ui.localize("ResetGui"));
                    //     })
                    //     .clicked()
                    // {
                    //     ui.memory_mut(|memory| {
                    //         *memory = Default::default();
                    //     });
                    //     ui.ctx().set_localizations();
                    //     *self = Default::default();
                    // }
                    // ui.separator();
                    // // Settings
                    // if ui
                    //     .button(RichText::new(GEAR).size(ICON_SIZE))
                    //     .on_hover_ui(|ui| {
                    //         ui.set_max_width(ui.spacing().tooltip_width);
                    //         ui.label(ui.localize("Settings"));
                    //     })
                    //     .clicked()
                    // {
                    //     state.windows.open_settings ^= true;
                    // }

                    ui.separator();
                    // Save
                    if ui
                        .button(RichText::new(FLOPPY_DISK).size(ICON_SIZE))
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("Save"));
                        })
                        .clicked()
                    {
                        let _ = self.save(ui, state);
                    }
                    ui.separator();
                    // Edit
                    if ui
                        .button(RichText::new(PENCIL).size(ICON_SIZE))
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("Edit"));
                        })
                        .clicked()
                    {
                        state.windows.open_edit ^= true;
                    }
                    ui.separator();
                    // Locale
                    LocaleButton::new()
                        .size(ICON_SIZE)
                        .ui(ui)
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("Language"));
                        });
                    ui.separator();
                    // About
                    if ui
                        .button(RichText::new(INFO).size(ICON_SIZE))
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("About"));
                        })
                        .clicked()
                    {
                        state.windows.open_about ^= true;
                    }
                    ui.separator();
                });
            });
        });
    }

    #[instrument(skip_all, err)]
    fn save(&mut self, ui: &Ui, state: &mut State) -> Result<()> {
        println!("SAVE");

        // let id = &self.data.current;
        // let data = df!(
        //     "Fruit" => ["Apple", "Apple", "Pear"],
        //     "Color" => ["Red", "Yellow", "Green"]
        // )?;
        // let mut meta = BTreeMap::new();
        // meta.insert("Name".to_string(), "The NAME".to_string());
        // meta.insert("Authors".to_string(), "value".to_string());
        // let frame = MetaDataFrame { meta, data };
        // export::ron::save(&frame, "name.ron")?;

        // let serialized = ron::ser::to_string_pretty(&frame, PrettyConfig::default())?;
        // println!("serialized: {serialized:#}");
        // let deserialized = ron::de::from_str::<DataFrame>(&serialized)?;
        // println!("deserialized: {deserialized}");
        Ok(())
    }
}

// Windows
impl App {
    fn windows(&mut self, ui: &Ui, state: &mut State) {
        self.about(ui, state);
        self.edit(ui, state);
        self.settings(ui, state);
    }

    fn about(&mut self, ui: &Ui, state: &mut State) {
        Window::new(format!("{INFO} About"))
            .id(Id::new(ID_SOURCE).with("About"))
            .open(&mut state.windows.open_about)
            .show(ui, |ui| {
                About::ui(ui);
            });
    }

    fn edit(&mut self, ui: &Ui, state: &mut State) {
        Window::new(format!("{PENCIL} Edit"))
            .id(Id::new(ID_SOURCE).with("Edit"))
            .open(&mut state.windows.open_edit)
            .show(ui, |ui| {
                // Settings::new().ui(ui);
            });
    }

    fn settings(&mut self, ui: &Ui, state: &mut State) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .id(Id::new(ID_SOURCE).with("Settings"))
            .open(&mut state.windows.open_settings)
            .show(ui, |ui| {
                state.settings.show(ui);
            });
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let mut state = State::load(ui, Id::new(ID_SOURCE).with(STATE));
        // Pre update
        self.panels(ui, &mut state);
        self.windows(ui, &mut state);
        // Post update
        state.store(ui, Id::new(ID_SOURCE).with(STATE));
    }
}

mod data;
mod state;
// mod export;

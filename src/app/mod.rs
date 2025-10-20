use std::{collections::BTreeMap, io::Cursor, sync::Arc};

use self::state::{About, Settings, State, Windows};
use crate::{app::data::Data, localization::ContextExt as _};
use anyhow::Result;
use eframe::{APP_KEY, CreationContext, Storage, get_value, set_value};
use egui::{
    Align, CentralPanel, Context, FontDefinitions, Frame, Id, Label, Layout, MenuBar, RichText,
    ScrollArea, SidePanel, Sides, TopBottomPanel, Ui, Widget, Window, warn_if_debug_build,
};
use egui_ext::{
    LightDarkButton,
    download::{NONE, download},
};
use egui_l20n::{UiExt as _, ui::locale_button::LocaleButton};
use egui_phosphor::{
    Variant, add_to_fonts,
    regular::{
        ARROWS_CLOCKWISE, FILE, GEAR, INFO, PENCIL, SIDEBAR_SIMPLE, SLIDERS_HORIZONTAL, TRASH,
    },
};
use polars::prelude::*;
use ron::ser::{PrettyConfig, to_string_pretty, to_writer};
use serde::{Deserialize, Serialize};
use tracing::instrument;

const ICON_SIZE: f32 = 32.0;
const ID_SOURCE: &str = "FAN";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Panels
    left_panel: bool,
    // Data
    data: Data,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left_panel: true,
            data: Data::new(),
        }
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
    fn panels(&mut self, ctx: &Context, state: &mut State) {
        self.top_panel(ctx, state);
        self.bottom_panel(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
    }

    // Bottom panel
    fn bottom_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("BottomPanel").show(ctx, |ui| {
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
    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()))
            .show(ctx, |ui| {
                self.data.central(ui);
            });
    }

    // Left panel
    fn left_panel(&mut self, ctx: &Context) {
        SidePanel::left("LeftPanel")
            .resizable(true)
            .show_animated(ctx, self.left_panel, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.data.left(ui);
                });
            });
    }

    // Top panel
    fn top_panel(&mut self, ctx: &Context, state: &mut State) {
        TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    // Left panel
                    ui.toggle_value(
                        &mut self.left_panel,
                        RichText::new(SIDEBAR_SIMPLE).size(ICON_SIZE),
                    )
                    .on_hover_ui(|ui| {
                        ui.set_max_width(ui.spacing().tooltip_width);
                        ui.label(ui.localize("LeftPanel"));
                    });
                    ui.separator();
                    // Light/Dark
                    ui.light_dark_button(ICON_SIZE);
                    ui.separator();
                    // Reset
                    if ui
                        .button(RichText::new(ARROWS_CLOCKWISE).size(ICON_SIZE))
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("ResetGui"));
                        })
                        .clicked()
                    {
                        ui.memory_mut(|memory| {
                            *memory = Default::default();
                        });
                        ui.ctx().set_localizations();
                        *self = Default::default();
                    }
                    ui.separator();
                    // Settings
                    if ui
                        .button(RichText::new(GEAR).size(ICON_SIZE))
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("Settings"));
                        })
                        .clicked()
                    {
                        state.windows.open_settings ^= true;
                    }
                    ui.separator();
                    // Save
                    if ui
                        .button(RichText::new(FILE).size(ICON_SIZE))
                        .on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(ui.localize("Save"));
                        })
                        .clicked()
                    {
                        let _ = self.save(ctx, state);
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
    fn save(&mut self, ctx: &Context, state: &mut State) -> Result<()> {
        println!("SAVE");

        let id = &self.data.current;
        let data = df!(
            "Fruit" => ["Apple", "Apple", "Pear"],
            "Color" => ["Red", "Yellow", "Green"]
        )?;
        let mut meta = BTreeMap::new();
        meta.insert("Name".to_string(), "The NAME".to_string());
        meta.insert("Authors".to_string(), "value".to_string());
        let frame = (meta, data);
        let serialized = ron::ser::to_string_pretty(&frame, PrettyConfig::default())?;
        println!("serialized: {serialized:#}");
        // let deserialized = ron::de::from_str::<DataFrame>(&serialized)?;
        // println!("deserialized: {deserialized}");
        Ok(())
    }
}

// Windows
impl App {
    fn windows(&mut self, ctx: &Context, state: &mut State) {
        self.about(ctx, state);
        self.edit(ctx, state);
        self.settings(ctx, state);
    }

    fn about(&mut self, ctx: &Context, state: &mut State) {
        Window::new(format!("{INFO} About"))
            .id(Id::new(ID_SOURCE).with("About"))
            .open(&mut state.windows.open_about)
            .show(ctx, |ui| {
                About::ui(ui);
            });
    }

    fn edit(&mut self, ctx: &Context, state: &mut State) {
        Window::new(format!("{PENCIL} Edit"))
            .id(Id::new(ID_SOURCE).with("Edit"))
            .open(&mut state.windows.open_edit)
            .show(ctx, |ui| {
                // Settings::new().ui(ui);
            });
    }

    fn settings(&mut self, ctx: &Context, state: &mut State) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .id(Id::new(ID_SOURCE).with("Settings"))
            .open(&mut state.windows.open_settings)
            .show(ctx, |ui| {
                state.settings.ui(ui);
            });
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut state = State::load(ctx, Id::new(ID_SOURCE).with("State"));
        // Pre update
        self.panels(ctx, &mut state);
        self.windows(ctx, &mut state);
        // Post update
        state.store(ctx, Id::new(ID_SOURCE).with("State"));
    }
}

mod data;
mod export;
mod state;

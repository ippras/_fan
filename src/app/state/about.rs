use egui::{Label, Response, RichText, Sense, TextStyle, Ui};
use egui_phosphor::regular::{COPYRIGHT, GITHUB_LOGO, GLOBE, WARNING};
use serde::{Deserialize, Serialize};

/// About window
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct About;

impl About {
    pub(crate) fn ui(ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            let version = env!("CARGO_PKG_VERSION");
            ui.heading(format!("FAN {version}"));
            ui.label("Fatty acid names");
            // Links
            ui.separator();
            ui.collapsing(RichText::new("Links").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(GLOBE).heading())
                        .on_hover_text("web");
                    ui.hyperlink_to(
                        "https://ippras.github.io/fan",
                        "https://ippras.github.io/fan",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new(GITHUB_LOGO).heading())
                        .on_hover_text("github.com");
                    ui.hyperlink_to(
                        "https://github.com/ippras/fan",
                        "https://github.com/ippras/fan",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new(WARNING).heading())
                        .on_hover_text("report an issue");
                    ui.hyperlink_to(
                        "https://github.com/ippras/fan/issues",
                        "https://github.com/ippras/fan/issues",
                    );
                });
            });
            // Copyright
            ui.separator();
            ui.horizontal(|ui| {
                let width =
                    ui.fonts(|fonts| fonts.glyph_width(&TextStyle::Body.resolve(ui.style()), ' '));
                ui.spacing_mut().item_spacing.x = width;
                ui.label(COPYRIGHT);
                ui.label("2024");
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add(Label::new("Giorgi Kazakov").sense(Sense::click()));
                ui.spacing_mut().item_spacing.x = width;
                ui.label(",");
                ui.add(Label::new("Roman Sidorov").sense(Sense::click()));
            });
        })
        .response
    }
}

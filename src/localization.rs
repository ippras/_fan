use egui::Context;
use egui_l20n::{ContextExt as _, Localization};

/// Extension methods for [`Context`]
pub(crate) trait ContextExt {
    fn set_localizations(&self);
}

impl ContextExt for Context {
    fn set_localizations(&self) {
        self.set_localization(
            locales::EN_US,
            Localization::new(locales::EN_US).with_sources(sources::EN_US),
        );
        self.set_localization(
            locales::JP_JP,
            Localization::new(locales::JP_JP).with_sources(sources::JP_JP),
        );
        self.set_localization(
            locales::RU_RU,
            Localization::new(locales::RU_RU).with_sources(sources::RU_RU),
        );
        self.set_language_identifier(locales::EN_US)
    }
}

mod locales {
    use egui_l20n::{LanguageIdentifier, langid};

    pub(super) const EN_US: LanguageIdentifier = langid!("en-US");
    pub(super) const JP_JP: LanguageIdentifier = langid!("jp-JP");
    pub(super) const RU_RU: LanguageIdentifier = langid!("ru-RU");
}

mod sources {
    use crate::asset;

    pub(super) const EN_US: &[&str] = &[
        asset!("/ftl/en-US/main.ftl"),
        asset!("/ftl/en-US/aocs.org.ftl"),
        // asset!("/ftl/en-US/aocs.org.ext.ftl"),
    ];

    pub(super) const JP_JP: &[&str] = &[
        asset!("/ftl/jp-JP/main.ftl"),
        asset!("/ftl/en-US/aocs.org.ftl"),
        // asset!("/ftl/en-US/aocs.org.ext.ftl"),
    ];

    pub(super) const RU_RU: &[&str] = &[
        asset!("/ftl/ru-RU/main.ftl"),
        asset!("/ftl/en-US/aocs.org.ftl"),
        // asset!("/ftl/en-US/aocs.org.ext.ftl"),
    ];
}

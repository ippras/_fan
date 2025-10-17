use egui::Context;
use egui_l10n::{ContextExt as _, Localization};

/// Extension methods for [`Context`]
pub(crate) trait ContextExt {
    fn set_localizations(&self);
}

impl ContextExt for Context {
    fn set_localizations(&self) {
        self.set_localization(
            locales::EN,
            Localization::new(locales::EN)
                .with_sources(sources::EN)
                .with_sources(fatty_acid_names::l10n::EN),
        );
        self.set_localization(
            locales::JP,
            Localization::new(locales::JP)
                .with_sources(sources::JP)
                .with_sources(fatty_acid_names::l10n::EN),
        );
        self.set_localization(
            locales::RU,
            Localization::new(locales::RU)
                .with_sources(sources::RU)
                .with_sources(fatty_acid_names::l10n::RU),
        );
        self.set_language_identifier(locales::EN)
    }
}

mod locales {
    use egui_l10n::{LanguageIdentifier, langid};

    pub(super) const EN: LanguageIdentifier = langid!("en");
    pub(super) const JP: LanguageIdentifier = langid!("jp");
    pub(super) const RU: LanguageIdentifier = langid!("ru");
}

mod sources {
    use egui_l10n::ftl;

    pub(super) const EN: &[&str] = &[
        ftl!("en-US/main.ftl"),
        // ftl!("en-US/aocs.org.ftl"),
        // ftl!("en-US/aocs.org.ext.ftl"),
    ];

    pub(super) const JP: &[&str] = &[
        ftl!("jp-JP/main.ftl"),
        // ftl!("en-US/aocs.org.ftl"),
        // ftl!("en-US/aocs.org.ext.ftl"),
    ];

    pub(super) const RU: &[&str] = &[
        ftl!("ru-RU/main.ftl"),
        // ftl!("ru-RU/aocs.org.ftl"),
        // ftl!("en-US/aocs.org.ftl"),
        // ftl!("en-US/aocs.org.ext.ftl"),
    ];
}

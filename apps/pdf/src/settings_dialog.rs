//! Settings as a card over the window: the groups listed on the left, the
//! chosen group's options on the right, and a search across all of them.

use crate::sections::settings;
use crate::{widgets, window};
use gtk::prelude::*;
use nyx_ui::settings_card::{self, About, Spec, Standard};

pub use nyx_ui::settings_card::{escape, fit, focus_search, is_open};

/// Each settings group's icon and one-line summary in the dialog's list.
const GROUPS: &[(&str, &str, &str)] = &[
    ("Viewing", "npdf-document-symbolic", "Zoom, layout, reading"),
    ("Editing", "document-edit-symbolic", "Markup, your name, signatures"),
    ("Files", "folder-documents-symbolic", "Recent files, opening"),
];

const SPEC: Spec = Spec {
    groups: GROUPS,
    search_text: None,
    standard: Some(Standard {
        app: "pdf",
        about: About {
            name: "PDF",
            icon: "io.github.design_nexus.nyx.Pdf",
            version: env!("CARGO_PKG_VERSION"),
            description: "Read, annotate, fill, sign and rearrange PDFs.",
            licence: "MIT",
            website: "https://github.com/design-nexus/nyx-pdf",
            issues: "https://github.com/design-nexus/nyx-pdf/issues",
            credits: &[("Poppler", "rendering")],
        },
        shortcuts: window::SHORTCUTS,
        reduce_motion: || crate::prefs::get().reduce_motion,
        set_reduce_motion: |on| crate::prefs::update(|p| p.reduce_motion = on),
    }),
};

pub fn open() {
    let (Some(overlay), Some(window)) = (window::overlay(), window::window()) else { return };
    settings_card::open(&overlay, &window, SPEC, || {
        let page = widgets::page("settings");
        settings::build(&page);
        (page.root.upcast(), page.body)
    });
}

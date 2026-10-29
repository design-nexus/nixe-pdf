use crate::widgets::{self, Page};
use crate::{cmd, fmt, paths, prefs, theme};
use gtk::glib;
use gtk::prelude::*;

fn dir_size(dir: &std::path::Path) -> u64 {
    let mut total = 0;
    let Ok(rd) = std::fs::read_dir(dir) else { return 0 };
    for e in rd.flatten() {
        let Ok(m) = e.metadata() else { continue };
        total += if m.is_dir() { dir_size(&e.path()) } else { m.len() };
    }
    total
}

pub fn build(page: &Page) {
    let p = prefs::get();

    // ----- Viewing -----
    let g = page.group("Viewing");
    g.add(&widgets::segmented_row(
        "Zoom when a file opens",
        "Fit width keeps the text a comfortable size; fit page shows a whole page.",
        widgets::opts(&[("fit-width", "Fit width"), ("fit-page", "Fit page"), ("100", "100%")]),
        &p.default_zoom,
        |v| prefs::update(|p| p.default_zoom = v),
    ));
    g.add(&widgets::segmented_row(
        "Page colors",
        "Recolor pages to the theme for reading at night. Pictures are recolored too.",
        widgets::opts(&[("off", "Off"), ("invert", "Invert"), ("tint", "Theme tint")]),
        &p.page_colors,
        |v| {
            prefs::update(|p| p.page_colors = v);
            theme::apply();
        },
    ));
    let (r, _) = widgets::switch_row("Continuous scrolling", "Pages run one after another. Off shows one page at a time.", p.continuous, |on| {
        prefs::update(|p| p.continuous = on);
        crate::viewer::relayout();
    });
    g.add(&r);
    g.add(&widgets::segmented_row(
        "Pages side by side",
        "Two pages at a time, or like a book with page one on its own. Also in the layout menu above the pages.",
        widgets::opts(&[("single", "Single"), ("pairs", "Two pages"), ("book", "Book")]),
        &p.spread,
        |v| {
            prefs::update(|p| p.spread = v);
            crate::viewer::rebuild();
        },
    ));
    let (r, _) = widgets::switch_row("Remember the page", "Open each file on the page you left it on.", p.remember_page, |on| {
        prefs::update(|p| p.remember_page = on)
    });
    g.add(&r);

    // ----- Editing -----
    let g = page.group("Editing");
    let author = gtk::Entry::new();
    author.set_text(&p.author);
    author.set_placeholder_text(Some("Your name"));
    author.set_width_chars(22);
    author.connect_changed(|e| {
        let text = e.text().to_string();
        prefs::update(|p| p.author = text);
    });
    g.add(&widgets::row("Author name", "Stored on the notes and markup you add.", Some(author.upcast_ref())));
    g.add(&widgets::row("Highlight color", "The color new highlights start with.", Some(crate::viewer::colour::pref_row().upcast_ref())));
    let sigs = paths::signatures_dir();
    let n = std::fs::read_dir(&sigs).map(|d| d.flatten().count()).unwrap_or(0);
    let desc = format!("{} saved. They live in <tt>{}</tt>.", fmt::count(n, "signature", "signatures"), glib::markup_escape_text(&paths::pretty(&sigs)));
    let (r, _) = widgets::button_row("Signatures", &desc, "Open folder", move |_| {
        let _ = std::fs::create_dir_all(&sigs);
        cmd::spawn(&["xdg-open", &sigs.to_string_lossy()]);
    });
    g.add(&r);

    // ----- Files -----
    let g = page.group("Files");
    let cache = paths::cache_dir();
    let describe = move || {
        format!(
            "Library covers, and copies of files left from earlier runs (with any changes that weren't saved). {} now.",
            fmt::size(dir_size(&cache))
        )
    };
    let (r, _) = widgets::button_row("Clear cache", &describe(), "Clear", move |b| {
        crate::sections::library::clear_covers();
        let n = crate::doc::clear_leftovers();
        crate::window::toast(&if n == 0 { "Cleared the covers.".to_string() } else { format!("Cleared the covers and {}.", fmt::count(n, "old copy", "old copies")) });
        // Update the size in the description.
        if let Some(desc) = b.parent().and_then(|row| row.first_child()).and_then(|text| text.last_child()).and_downcast::<gtk::Label>() {
            desc.set_text(&describe());
        }
    });
    g.add(&r);
    g.note("Open a file from a terminal or a binding with <tt>pdf FILE</tt>, and jump to a page with <tt>pdf FILE --page 12</tt>.");
}

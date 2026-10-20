//! Russian, one table per part of the app, keyed by the English text.

mod common;
mod flasher;
mod pages;
mod settings;
mod tray;

/// The Russian for `en`, if there is one.
pub(super) fn lookup(en: &str) -> Option<&'static str> {
    common::lookup(en)
        .or_else(|| settings::lookup(en))
        .or_else(|| flasher::lookup(en))
        .or_else(|| pages::lookup(en))
        .or_else(|| tray::lookup(en))
}

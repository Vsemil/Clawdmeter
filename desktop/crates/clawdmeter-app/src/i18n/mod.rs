//! The app's interface language: the same `lang` setting as the device's
//! (`en`/`ru`). English text is the key: `tr("Device")` returns it unchanged
//! in English and its translation otherwise, so a string with no
//! translation yet still shows, in English. `tests::every_string_is_translated`
//! scans the sources and fails on a `tr`/`trf` literal with none.

use clawdmeter_core::config::Lang;
use std::sync::atomic::{AtomicU8, Ordering};

mod ru;

static LANG: AtomicU8 = AtomicU8::new(0);

/// Switch the interface language (on every settings load).
pub fn set(lang: Lang) {
    LANG.store(lang as u8, Ordering::Relaxed);
}

/// The interface language.
pub fn lang() -> Lang {
    if LANG.load(Ordering::Relaxed) == Lang::Ru as u8 {
        Lang::Ru
    } else {
        Lang::En
    }
}

/// `en` in the interface language.
pub fn tr(en: &'static str) -> &'static str {
    match lang() {
        Lang::En => en,
        Lang::Ru => ru::lookup(en).unwrap_or(en),
    }
}

/// [`tr`] of a template, each `{}` filled with the next of `args`.
pub fn trf(en: &'static str, args: &[&dyn std::fmt::Display]) -> String {
    let mut out = String::new();
    let mut args = args.iter();
    let mut parts = tr(en).split("{}").peekable();
    while let Some(part) = parts.next() {
        out.push_str(part);
        if parts.peek().is_some() {
            if let Some(a) = args.next() {
                out.push_str(&a.to_string());
            }
        }
    }
    out
}

/// A message the core composed from fixed sentences (an account's usage
/// error, `UsageError::user_message` / `usage::paused_message`), translated
/// sentence by sentence; a sentence with no translation stays as it is.
pub fn core_message(msg: &str) -> String {
    if lang() == Lang::En {
        return msg.to_owned();
    }
    if let Some(t) = msg.strip_prefix("Usage paused by Anthropic until ") {
        return trf("Usage paused by Anthropic until {}.", &[&t.trim_end_matches('.')]);
    }
    let mut out = Vec::new();
    for sentence in msg.split_inclusive(". ").map(str::trim) {
        let minutes = sentence
            .strip_prefix("Trying again in about ")
            .and_then(|r| r.split(' ').next())
            .and_then(|n| n.parse::<u32>().ok());
        out.push(match minutes {
            Some(m) => trf("Trying again in about {} min.", &[&m]),
            None => ru::lookup(sentence).unwrap_or(sentence).to_owned(),
        });
    }
    out.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_messages_translate_by_sentence() {
        assert_eq!(ru::lookup("Couldn't reach Claude."), Some("Не удалось связаться с Claude."));
        assert!(ru::lookup("Trying again in about {} min.").is_some());
        assert!(ru::lookup("Usage paused by Anthropic until {}.").is_some());
    }

    #[test]
    fn templates_fill_in_order() {
        set(Lang::En);
        assert_eq!(trf("{} of {}", &[&1, &"two"]), "1 of two");
    }

    /// Every `tr("…")` / `trf("…"` literal in the app's sources has a Russian
    /// translation, and every translation's `{}` count matches its key's.
    #[test]
    fn every_string_is_translated() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let own = root.join("i18n");
        let mut missing = Vec::new();
        let mut stack = vec![root];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().and_then(|e| e.to_str()) != Some("rs") || path.starts_with(&own) {
                    continue;
                }
                let src = std::fs::read_to_string(&path).unwrap();
                for key in literals(&src) {
                    match ru::lookup(&key) {
                        None => missing.push(format!("{}: {key:?}", path.display())),
                        Some(t) if t.matches("{}").count() != key.matches("{}").count() => {
                            missing.push(format!("{}: {key:?} has a different {{}} count", path.display()))
                        }
                        Some(_) => {}
                    }
                }
            }
        }
        assert!(missing.is_empty(), "untranslated:\n{}", missing.join("\n"));
    }

    /// The string literals passed straight to `tr(` / `trf(`, unescaped.
    fn literals(src: &str) -> Vec<String> {
        let mut out = Vec::new();
        for start in ["tr(\"", "trf(\""] {
            let mut rest = src;
            while let Some(i) = rest.find(start) {
                let before = rest[..i].chars().last();
                rest = &rest[i + start.len()..];
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                    continue; // e.g. `attr("`
                }
                let mut s = String::new();
                let mut chars = rest.chars();
                while let Some(c) = chars.next() {
                    match c {
                        '"' => break,
                        '\\' => match chars.next() {
                            Some('n') => s.push('\n'),
                            // A `\`-newline continuation drops the newline and the next line's indent.
                            Some('\n') => {
                                while chars.clone().next().is_some_and(char::is_whitespace) {
                                    chars.next();
                                }
                            }
                            Some(o) => s.push(o),
                            None => {}
                        },
                        c => s.push(c),
                    }
                }
                out.push(s);
            }
        }
        out
    }
}

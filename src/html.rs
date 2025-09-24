//! Safely construct HTML from templates and user data.

use std::collections::{HashMap};
use std::{fmt};

pub use html_escape::{encode_text as html_escape};

/// Safely include a value in some HTML, replacing special characters with
/// escape sequences.
///
/// At least the following implementations are provided:
/// - [`&'static str`] - No escaping required (already correctly escaped).
/// - [`String`] - Apply `html_escape`.
/// - [`Url`] - Apply `html_escape` (already %-escaped).
/// - [Box<dyn Escape>] - Concatenate the pieces.
/// - Template - interpolate the pieces into a template.
pub trait Escape {
    /// Append the escaped version of `self` to `out`.
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result;
}

impl Escape for &'static str {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result { out.write_str(self) }
}

impl Escape for String {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        out.write_str(html_escape(&self).as_ref())
    }
}

impl Escape for url::Url {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        out.write_str(html_escape(self.as_str()).as_ref())
    }
}

impl Escape for [Box<dyn Escape>] {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        for item in self { item.escape(out)?; }
        Ok(())
    }
}

// ----------------------------------------------------------------------------

/// An HTML fragment constructed by interpolating variables into a template.
pub struct Template(
    /// The template string: the invariant part of the HTML. This should
    /// already be correctly escaped. Consider using [`std::include_str`].
    ///
    /// `{variable}` is replaced by the escaped version of `variable`.
    &'static str,

    /// Variables to be escaped and interpolated into the template.
    Box<[(String, Box<dyn Escape>)]>,
);

impl Escape for Template {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        todo!();
    }
}

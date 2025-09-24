use std::{fmt};
use std::error::{Error};
use std::ffi::{OsStr, OsString};

pub mod content_types;

pub mod html;

mod server;
pub use server::{
    html_escape, Url,
    HttpOkay, HttpError, Handle, start,
};

// ----------------------------------------------------------------------------

/// Given `"foo.BAR"` and `"bar"` returns `Some("foo")`.
pub fn remove_extension<'a>(filename: &'a str, extension: &str) -> Option<&'a str> {
    if let Some(index) = filename.len().checked_sub(".".len() + extension.len()) {
        if let Some((ret, tail)) = filename.split_at_checked(index) {
            let mut tail = tail.chars();
            if let Some('.') = tail.next() {
                if extension.eq_ignore_ascii_case(tail.as_str()) { return Some(ret); }
            }
        }
    }
    None
}

// ----------------------------------------------------------------------------

/// `Error` returned by `validate_name()` if it doesn't like the filename.
#[derive(Debug)]
pub struct DubiousFilename(OsString);

impl fmt::Display for DubiousFilename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filename {:?} contains unusual characters; only letters, digits and \"_-.\" are allowed", self.0)
    }
}

impl Error for DubiousFilename {}

/// If `s` only contains alphanumeric characters and characters in `_-.`,
/// returns it unchanged.
///
/// There are perfectly good filenames that do not satisfy this criterion, but
/// those that do are unlikely to need to be escaped in any context. This
/// criterion is satisfied by many common filenames, including auto-generated
/// filenames that are based on dates, hashes or sequence numbers.
pub fn validate_name(s: &OsStr) -> Result<&str, DubiousFilename> {
    for b in s.as_encoded_bytes() {
        match b {
            b'0' .. b'9' => {},
            b'A' .. b'Z' => {},
            b'a' .. b'z' => {},
            b'_' | b'.' | b'-' => {}
            _ => { return Err(DubiousFilename(s.to_owned())); }
        }
    }
    Ok(s.to_str().unwrap())
}

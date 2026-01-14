//! Safely construct HTML from templates and user data.

use std::{fmt, iter};

pub use html_escape::{encode_text as escape};

/// Safely include a value in some HTML, replacing special characters with
/// escape sequences.
///
/// Types that implement [`std::fmt::Display`], including strings, numbers, and
/// other primitive types, [`Box<str>`] and [`Rc<str>`], will be HTML-escaped.
///
/// This trait is dyn-safe, i.e. values of type `dyn Escape` are allowed.
///
/// [`Rc<str>`]: std::rc::Rc<str>
/// ```
/// use petite_http::html::{Escape, Raw};
/// assert_eq!("this & that".to_html(), Raw("this &amp; that"));
/// assert_eq!(String::from("this & that").to_html(), Raw("this &amp; that"));
/// assert_eq!(Box::new("this & that").to_html(), Raw("this &amp; that"));
/// assert_eq!(std::rc::Rc::new("this & that").to_html(), Raw("this &amp; that"));
/// ```
pub trait Escape: fmt::Debug {
    /// Append the escaped version of `self` to `out`.
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result;

    /// Convert `self` to HTML.
    fn to_html(&self) -> Raw<String> {
        let mut ret = String::new();
        self.escape(&mut ret).unwrap(); // <String as fmt::Write> is safe.
        Raw(ret)
    }
}

impl<T: fmt::Display + fmt::Debug> Escape for T {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        out.write_str(escape(&format!("{}", self)).as_ref())
    }
}

impl Escape for Box<dyn Escape> {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result { (**self).escape(out) }
}

// ----------------------------------------------------------------------------

/// Assert that `S`'s string representation already includes all necessary
/// escape sequences and is a valid HTML fragment.
#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
pub struct Raw<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> Raw<S> {
    /// Maps a `Raw<S>` to a `Raw<T>` by applying the function `f`.
    pub fn map<T: AsRef<str>>(self, f: impl FnOnce(S) -> T) -> Raw<T> {
        Raw(f(self.0))
    }

    /// Converts from `&Raw<S>` to `Raw<&S>`.
    pub fn as_ref(&self) -> Raw<&S> { Raw(&self.0) }
}

impl<S: AsRef<str>, T: AsRef<str>> PartialEq<Raw<T>> for Raw<S> where S: PartialEq<T> {
    fn eq(&self, other: &Raw<T>) -> bool { self.0 == other.0 }
}

impl<S: AsRef<str> + fmt::Debug> Escape for Raw<S> {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        out.write_str(self.0.as_ref())
    }
}

/// `html::include_html!("filename")` expands to a [`Raw`] containing the
/// contents of file `filename`. This is a convenient way of compiling
/// quantities of static HTML into your program.
#[macro_export]
macro_rules! include_html {
    ($filename:expr) => ( crate::html::Raw(include_str!($filename)) )
}

// ----------------------------------------------------------------------------

/// Concatenates zero or more values, not necessarily of the same type, that
/// can be safely included in HTML.
///
/// ```
/// use petite_http::html::{Escape, Raw, Concat};
/// assert_eq!(
///     Concat(Box::new([
///         Box::new(Concat(Box::new([Box::new("a"), Box::new('='), Box::new(1)]))),
///         Box::new('&'),
///         Box::new(Concat(Box::new([Box::new("b"), Box::new('='), Box::new(2)]))),
///     ])).to_html(),
///     Raw("a=1&amp;b=2"),
/// )
/// ```
#[derive(Debug)]
pub struct Concat(pub Box<[Box<dyn Escape>]>);

impl Escape for Concat {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        for item in &self.0 { item.escape(out)?; }
        Ok(())
    }
}

impl<A: 'static + Escape> iter::FromIterator<A> for Concat {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        Self(iter.into_iter().map(|item| Box::new(item) as Box<dyn Escape>).collect())
    }
}

// ----------------------------------------------------------------------------

/// An HTML fragment constructed by interpolating variables into a template.
///
/// ```
/// use petite_http::html::{Escape, Raw, Template};
/// let observed = Template(
///         "<p>{noun} is {adjective}!</p>",
///         Box::new([
///             ("noun", Box::new("<em>User data</em>")),
///             ("adjective", Box::new("escaped")),
///         ]),
///    ).to_html();
/// let expected = Raw("<p>&lt;em&gt;User data&lt;/em&gt; is escaped!</p>");
/// assert_eq!(observed, expected);
/// ```
#[derive(Debug)]
pub struct Template(
    /// The template string: the invariant part of the HTML. This should
    /// already be correctly escaped. Consider using [`std::include_str`].
    ///
    /// `{variable}` is replaced by the escaped version of `variable`.
    /// '{{' is replaced by '{'.
    pub &'static str,

    /// Variables to be escaped and interpolated into the template.
    pub Box<[(&'static str, Box<dyn Escape>)]>,
);

impl Escape for Template {
    fn escape(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        let mut s = self.0;
        while let Some((head, tail)) = s.split_once('{') {
            out.write_str(head)?;
            let mut chars = tail.chars();
            if let Some('{') = chars.next() {
                out.write_char('{')?;
                s = chars.as_str();
            } else {
                if let Some((variable, tail)) = tail.split_once('}') {
                    if super::validate_name(variable.as_ref()).is_err() {
                        panic!("Malformed template variable '{variable}'");
                    }
                    let value = self.1.iter().find_map(
                        |(key, value)| if *key == variable { Some(value) } else { None }
                    ).unwrap_or_else(|| {
                        panic!("No value for template variable '{variable}'");
                    });
                    value.escape(out)?;
                    s = tail;
                } else { Err(fmt::Error)?; }
            }
        }
        out.write_str(s)
    }
}

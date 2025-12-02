use std::collections::{HashMap};

use petite_http::{self as ph, html, include_html, HttpOkay, HttpError};
use ph::content_types::{CSS};
use ph::html::{Escape, Concat, Template};

mod greet;
use greet::{Greet};

mod thank;
use thank::{Thank};

// ----------------------------------------------------------------------------

#[derive(Default, Debug, Clone)]
pub struct Demo {
    visitors: HashMap<String, String>,
}

impl Demo {
    fn guest(name: &String) -> Box<dyn Escape> {
        Box::new(Template(
            r#"<li><a href="/visitor/{url_name}">{name}</a></li>"#,
            Box::new([
                ("name", Box::new(name.clone())),
                ("url_name", Box::new(name.clone())), // FIXME: URL-encode
            ]),
        ))
    }

    fn guest_book(&self) -> Box<dyn Escape> {
        Box::new(Template(
            include_str!("guest_book.html"),
            Box::new([
                ("visitors", Box::new(Concat(self.visitors.keys().map(Self::guest).collect()))),
            ]),
        ))
    }

    fn visitor(&self, name: String) -> Box<dyn Escape> {
        let greeting = self.visitors.get(&name).unwrap().clone();
        Box::new(Template(
            include_str!("visitor.html"),
            Box::new([
                ("name", Box::new(name.clone())),
                ("greeting", Box::new(greeting.clone())),
                ("guest_book", self.guest_book()),
            ]),
        ))
    }
}

impl ph::Route for Demo {
    fn route(&mut self, path: &[String], callback: impl ph::Callback) -> ph::Result {
        let mut path_iter = path.into_iter();
        let Some(page) = path_iter.next() else {
            return Ok(HttpOkay::Redirect("start".into()))
        };
        if page == "stylesheet.css" {
            return Ok(HttpOkay::Chars {data: include_str!("stylesheet.css").into(), content_type: CSS });
        } else if page == "start" {
            return Ok(HttpOkay::Html(Box::new(include_html!("start.html"))));
        } else if page == "greet" {
            return callback.handle_with(&mut Greet(self));
        } else if page == "thank" {
            return callback.handle_with(&mut Thank(self));
        } else if page == "visitor" {
            let Some(name) = path_iter.next() else {
                return Err(HttpError::Invalid)
            };
            if self.visitors.contains_key(name) {
                return Ok(HttpOkay::Html(self.visitor(name.into())));
            } else {
                return Err(HttpError::NotFound)
            }
        }
        return Err(HttpError::Invalid);
    }
}

// ----------------------------------------------------------------------------

fn main() {
    ph::start("localhost:8080".into(), None, Demo::default());
}

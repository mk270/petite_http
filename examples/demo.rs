use std::collections::{HashMap};

use petite_http::{self as ph, html, HttpOkay, HttpError, include_html};
use ph::content_types::{CSS};
use ph::html::{Escape, Concat, Template};

#[derive(Default, Debug, Clone)]
pub struct Params {
    pub name: String,
    pub greeting: String,
}

impl FromIterator<(String, String)> for Params {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut ret = Self::default();
        for (key, value) in iter {
            if key == "name" { ret.name = value; }
            else if key == "greeting" { ret.greeting = value; }
        }
        ret
    }
}

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

    fn greet(&self, params: Params) -> Box<dyn Escape> {
        let greeting = self.visitors.get(&params.name).unwrap().clone();
        Box::new(Template(
            include_str!("greet.html"),
            Box::new([
                ("name", Box::new(params.name.clone())),
                ("greeting", Box::new(greeting)),
                ("guest_book", self.guest_book()),
            ]),
        ))
    }

    fn introduce(&self, params: Params) -> Box<dyn Escape> {
        Box::new(Template(
            include_str!("introduce.html"),
            Box::new([
                ("name", Box::new(params.name.clone())),
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

impl ph::Handle for Demo {
    type Params = Params;

    fn handle_get(&mut self, path: Vec<String>, params: Params) -> ph::Result {
        let mut path_iter = path.into_iter();
        if let Some(page) = path_iter.next() {
            if page == "stylesheet.css" {
                return Ok(HttpOkay::Chars {data: include_str!("stylesheet.css").into(), content_type: CSS });
            } else if page == "start" {
                return Ok(HttpOkay::Html(Box::new(include_html!("start.html"))));
            } else if page == "greet" {
                if "" != params.name {
                    if self.visitors.contains_key(&params.name) {
                        return Ok(HttpOkay::Html(self.greet(params)));
                    } else {
                        return Ok(HttpOkay::Html(self.introduce(params)));
                    }
                }
            } else if page == "thank" {
                if "" != params.name && "" != params.greeting {
                    self.visitors.insert(params.name, params.greeting);
                    return Ok(HttpOkay::Html(Box::new(include_html!("thank.html"))));
                }
            } else if page == "visitor" {
                if let Some(name) = path_iter.next() {
                    if self.visitors.contains_key(&name) {
                        return Ok(HttpOkay::Html(self.visitor(name)));
                    } else {
                        return Err(HttpError::NotFound)
                    }
                }
            }
            Err(HttpError::Invalid)
        } else {
            Ok(HttpOkay::Redirect("start".into()))
        }
    }
}

fn main() {
    ph::start("localhost:8080".into(), None, Demo::default());
}

use petite_http::{self as ph, html, HttpOkay, HttpError};
use html::{Escape, Template};

#[derive(Default, Debug, Clone)]
pub struct Params {name: String}

impl FromIterator<(String, String)> for Params {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut ret = Self::default();
        for (key, value) in iter {
            if key == "name" { ret.name = value; }
        }
        ret
    }
}

// ----------------------------------------------------------------------------

/// [`ph::Handle`]s the `/greet` URL space.
pub struct Greet<'a>(pub &'a mut super::Demo);

impl<'a> Greet<'a> {
    fn greet(&self, params: Params) -> Box<dyn Escape> {
        let greeting = self.0.visitors[&params.name].clone();
        Box::new(Template(
            include_str!("greet.html"),
            Box::new([
                ("name", Box::new(params.name.clone())),
                ("greeting", Box::new(greeting)),
                ("guest_book", self.0.guest_book()),
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
}

impl<'a> ph::Handle for Greet<'a> {
    type Params = Params;

    fn handle_get(
        &mut self,
        _path: Vec<String>,
        params: Self::Params,
    ) -> ph::Result {
        if "" != params.name {
            if self.0.visitors.contains_key(&params.name) {
                return Ok(HttpOkay::Html(self.greet(params)));
            } else {
                return Ok(HttpOkay::Html(self.introduce(params)));
            }
        }
        Err(HttpError::Invalid)
    }
}

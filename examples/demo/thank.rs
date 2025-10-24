use petite_http::{self as ph, HttpOkay, HttpError, include_html};

#[derive(Default, Debug, Clone)]
pub struct Params {name: String, greeting: String}

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

// ----------------------------------------------------------------------------

/// [`ph::Handle`]s the `/greet` URL space.
pub struct Thank<'a>(pub &'a mut super::Demo);

impl<'a> ph::Handle for Thank<'a> {
    type Params = Params;

    fn handle_get(
        &mut self,
        _path: &[String],
        params: Self::Params,
    ) -> ph::Result {
        if "" != params.name && "" != params.greeting {
            self.0.visitors.insert(params.name, params.greeting);
            return Ok(HttpOkay::Html(Box::new(include_html!("thank.html"))));
        }
        Err(HttpError::Invalid)
    }
}

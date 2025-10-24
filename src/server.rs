use std::{fmt};
use std::error::{Error};
use std::fs::{File};

use tiny_http::{Method, Request, Response, Header};

use url::{Url};

use super::{content_types, html};

/// A normal HTTP response.
#[derive(Debug)]
pub enum HttpOkay {
    /// A static file.
    File(File),

    /// Dynamic HTML.
    Html(Box<dyn html::Escape>),

    /// Dynamic character data.
    Chars {data: String, content_type: &'static [u8]},

    /// Dynamic binary data.
    Bytes {data: Vec<u8>, content_type: &'static [u8]},

    /// Temporary redirect. The client should immediately request the given
    /// URL, which is relative to the `base_url` of the [`Handle`].
    Redirect(String),
}

/// An erroneous HTTP response.
#[derive(Debug)]
pub enum HttpError {
    Invalid,
    NotFound,
    Error(Box<dyn Error>),
}

impl HttpError {
    pub fn new(e: impl 'static + Error) -> Self { Self::Error(e.into()) }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for HttpError {}

macro_rules! impl_httperror_from {
    ($e:ty) => {
        impl From<$e> for HttpError {
            fn from(e: $e) -> Self { Self::new(e) }
        }
    };
}

impl_httperror_from!(std::io::Error);
impl_httperror_from!(url::ParseError);
impl_httperror_from!(crate::DubiousFilename);

/// The return type of [`Handle::handle_get()`].
pub type Result = std::result::Result<HttpOkay, HttpError>;

// ----------------------------------------------------------------------------

/// Implement this to write your web application.
pub trait Handle {
    /// Represents the URL request parameters that are recognised by this
    /// Handle.
    ///
    /// A [`std::collections::HashMap`] is a possible choice, or you can
    /// provide something with more type-checking.
    type Params: FromIterator<(String, String)>;

    /// Called for each GET request.
    ///
    /// - path - The part of the requested URL relative to `base_url`.
    ///   For example if this Handle is at `http://example.com/foo` and
    ///   the client requests is `http://example.com/foo/bar/baz` then `path`
    ///   will be `["bar", "baz"]`.
    /// - params - the parsed URL request parameters.
    ///
    /// Note that the [`String`]s in `path` and `params` might contain special
    /// characters such as `/` and `?`, and non-ASCII characters. Be careful if
    /// you construct filesystem paths from these `String`s.
    fn handle_get(
        &mut self,
        path: &[String],
        params: Self::Params,
    ) -> self::Result;
}

// ----------------------------------------------------------------------------

/// An argument type of `Route::route`.
pub trait Callback {
    fn handle_with(self, handler: &mut impl Handle) -> self::Result;
}


pub trait Route {
    /// Examine the URL path and select a handler.
    ///
    /// If the path is sufficient to generate a response, just return it. If,
    /// say, the URL query parameters are needed to generate a response, then
    /// pass an implementation of [`Handle`] to `callback`.
    fn route(
        &mut self,
        path: &[String],
        callback: impl Callback,
    ) -> self::Result;
}

impl<H: Handle> Route for H {
    fn route(&mut self, _path: &[String], callback: impl Callback) -> self::Result {
        callback.handle_with(self)
    }
}

// ----------------------------------------------------------------------------

/// The name of the HTTP `Content-Type` header.
const CONTENT_TYPE: &'static [u8] = b"Content-Type";

/// The name of the HTTP `Location` header.
const LOCATION: &'static [u8] = b"Location";

/// Construct an HTTP header.
fn header(key: &'static [u8], value: &[u8]) -> tiny_http::Header {
    Header::from_bytes(key, value).unwrap() // depends only on data fixed at compile time
}

struct Server {
    /// Web server.
    pub server: tiny_http::Server,

    /// The local URL that `server` serves.
    pub server_url: Url,

    /// The publicly visible external URL, which may differ from `server_url`.
    pub base_url: Url,
}

impl Server {
    fn new(server_address: &str, base_url: Option<&str>) -> Self {
        let server_url = &format!("http://{}/", server_address);
        let base_url = base_url.unwrap_or_else(|| server_url);
        assert!(base_url.ends_with('/'));
        Server {
            server: tiny_http::Server::http(server_address).expect("Could not create the web server"),
            server_url: Url::parse(server_url).expect("Could not parse the server URL"),
            base_url: Url::parse(base_url).expect("Could not parse the base URL"),
        }
    }

    fn handle_request(
        &self,
        router: &mut impl Route,
        request: &mut Request,
    ) -> self::Result {
        let request_url = self.server_url.join(request.url())?;
        let relative_url = self.server_url.make_relative(&request_url).unwrap(); // By construction.
        println!("{} {}", request.remote_addr().unwrap().ip(), relative_url);
        // Parse the path segments.
        let mut path: Vec<String> = request_url.path_segments().ok_or(HttpError::Invalid)?.map(
            |s| url_escape::decode(s).into_owned()
        ).collect();
        if let Some(last) = path.last() {
            if "" == last { path.pop(); }
        }
        // Make a callback.
        struct Callback<'a> {
            path: &'a [String],
            method: &'a Method,
            url: Url,
        }
        impl<'a> self::Callback for Callback<'a> {
            fn handle_with(self, handler: &mut impl Handle) -> self::Result {
                // Parse the query parameters.
                let params = self.url.query_pairs().map(
                    |(key, value)| (
                        key.as_ref().into(),
                        value.as_ref().into(),
                    )
                ).collect();
                // Dispatch based on HTTP method.
                match self.method {
                    Method::Get => handler.handle_get(self.path, params),
                    _ => Err(HttpError::Invalid),
                }
            }
        }
        router.route(&*path, Callback {
            path: &*path,
            method: request.method(),
            url: request_url,
        })
    }

    /// Handle requests for ever.
    fn handle_requests(&self, mut router: impl Route) -> ! {
        for mut request in self.server.incoming_requests() {
            match self.handle_request(&mut router, &mut request) {
                Ok(HttpOkay::File(file)) => {
                    request.respond(Response::from_file(file))
                },
                Ok(HttpOkay::Html(text)) => {
                    let header = header(CONTENT_TYPE, content_types::HTML);
                    let html::Raw(escaped_text) = text.to_html();
                    request.respond(Response::from_string(escaped_text).with_header(header))
                },
                Ok(HttpOkay::Chars {data, content_type}) => {
                    let header = header(CONTENT_TYPE, content_type);
                    request.respond(Response::from_string(data).with_header(header))
                },
                Ok(HttpOkay::Bytes {data, content_type}) => {
                    let header = header(CONTENT_TYPE, content_type);
                    request.respond(Response::from_data(data).with_header(header))
                },
                Ok(HttpOkay::Redirect(relative_url)) => {
                    match self.base_url.join(&relative_url) {
                        Ok(absolute_url) => {
                            request.respond(
                                Response::from_string("Temporary Redirect").with_status_code(307)
                                    .with_header(header(LOCATION, absolute_url.as_str().as_bytes()))
                            )
                        },
                        Err(e) => {
                            println!("Error contructing URL for redirect: {}", e);
                            request.respond(Response::from_string("Server error").with_status_code(500))
                        },
                    }
                },
                Err(HttpError::Invalid) => {
                    request.respond(Response::from_string("Invalid request").with_status_code(400))
                },
                Err(HttpError::NotFound) => {
                    request.respond(Response::from_string("Not found").with_status_code(404))
                },
                Err(HttpError::Error(e)) => {
                    println!("Error: {}", e);
                    request.respond(Response::from_string("Server error").with_status_code(500))
                },
            }.unwrap_or_else(|e2| println!("IO Error: {}", e2));
        }
        unreachable!();
    }
}

/// Run for ever!
///
/// - server_address - E.g. "127.0.0.1:8082".
/// - base_url - The publicly visible URL of this web server, if any. It should
///   end with `/`. This is useful for constructing absolute URLs for HTTP redirects.
///   If `server_address` is public, `base_url` can be omitted.
/// - handler - Defines the web application.
pub fn start(server_address: String, base_url: Option<String>, router: impl Route) -> ! {
    let server = Server::new(&server_address, base_url.as_ref().map(AsRef::as_ref));
    println!("Listening on {}", server.server_url);
    server.handle_requests(router);
}

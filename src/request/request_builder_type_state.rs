// Type-state pattern - built over the consuming builder pattern
// https://www.youtube.com/watch?v=pwmIQzLuYl0
//
// Generics generate code per type (monomorphization), but make the code more reusable.
// This can increase the binary size.
// Moves in new() are optimised by the compiler.

use std::result::Result;
use std::error::Error;
use crate::request::Request;

// region: --- States
// We can't use an 'enum' because we need two different types.
// An enum is a single type. Enum variants are not types.
#[derive(Default, Clone)]
pub struct NoUrl;

#[derive(Default, Clone)]
pub struct Url(String);
// endregion: --- States

// Make the RequestBuilder generic over type U, which is the url type
#[derive(Default, Clone)]
pub struct RequestBuilder<U> {
    url: U,
    method: Option<String>,
    headers: Vec<(String, String)>,  // (name, value)
    body: Option<String>,
}

// Implement new() for the NoUrl type
impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

// Move RequestBuilder::build into a specialised case, and remove the run-time check:
impl RequestBuilder<Url> {
    pub fn build(self) -> Result<Request, Box<dyn Error>> {
        let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url: self.url.0,  // from Url(String), which is the specific generic type
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}

impl<U> RequestBuilder<U> {

    // Return type is now RequestBuilder<Url>
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn header(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

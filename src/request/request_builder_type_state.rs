// Type-state pattern - built over the consuming builder pattern
//

use std::result::Result;
use std::error::Error;
use crate::request::Request;

#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,  // (name, value)
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
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

    pub fn build(self) -> Result<Request, Box<dyn Error>> {
        let Some(url) = self.url else {
            return Err("No URL".into());
        };
        let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url,
            method,
            headers: self.headers,
            body: self.body,
        })
    }

}

// Non-consuming - internal cloning
// Best when no need to move the data to the target struct.
//
// Copy/paste target struct, and make all fields optional.
// Derive Default.
// Properties should not be public.

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

    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        //self.url = Some(url.into());
        let _ = self.url.insert(url.into());
        self
    }

    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        //self.method = Some(method.into());
        let _ = self.method.insert(method.into());
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        //self.body = Some(body.into());
        let _ = self.body.insert(body.into());
        self
    }

    pub fn header(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // Non-consuming pattern
    // Because this is &self, we have to clone the data
    // Do not use a &mut self because data is moved out of the builder
    // and therefore not available to subsequent calls - bad API.
    pub fn build(&self) -> Result<Request, Box<dyn Error>> {
        let Some(url) = self.url.as_ref() else {
            return Err("No URL".into());
        };
        let method = self.method.clone().unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url: url.to_string(),
            method,
            headers: self.headers.clone(),
            body: self.body.clone(),
        })
    }
}

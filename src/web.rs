// https://www.youtube.com/watch?v=Z_3WOSiYYFY

use std::result::Result;
use std::error::Error;

#[derive(Debug)]
pub struct Request {
    url: String,
    method :String,
    headers: Vec<(String, String)>,  // (name, value)
    body: Option<String>,
}

// Copy/paste target struct, and make all fields optional
// Derive Default.
// Properties should not be public.
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

    // Non-consuming - internal cloning
    // Best when no need to move the data to the target struct.

    // non-consuming
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        //self.url = Some(url.into());
        let _ = self.url.insert(url.into());
        self
    }

    // non-consuming
    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        //self.method = Some(method.into());
        let _ = self.method.insert(method.into());
        self
    }

    // non-consuming
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        //self.body = Some(body.into());
        let _ = self.body.insert(body.into());
        self
    }

    // non-consuming
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

    // Consuming
    pub fn url_consuming(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    // Consuming
    pub fn method_consuming(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    // Consuming
    pub fn body_consuming(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    // Consuming
    pub fn header_consuming(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // Consuming pattern - explicit clone, one fewer clone
    pub fn build_consuming(self) -> Result<Request, Box<dyn Error>> {
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

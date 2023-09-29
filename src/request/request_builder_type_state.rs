// Type-state pattern - built over the consuming builder pattern
// https://www.youtube.com/watch?v=pwmIQzLuYl0
//
// Generics generate code per type (monomorphization), but make the code more reusable.
// This can increase the binary size.
// Moves in new() are optimised by the compiler.

use crate::request::Request;
use core::marker::PhantomData;

// region: --- States
// We can't use an 'enum' because we need two different types.
// An enum is a single type. Enum variants are not types.
#[derive(Default, Clone)]
pub struct NoUrl;

#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct NoMethod;

#[derive(Default, Clone)]
pub struct Method(String);

// A sealed builder can only have .build() and .clone() called on it.
// Neither of these types have any data.
#[derive(Default, Clone)]
pub struct NotSealed;

#[derive(Default, Clone)]
pub struct Sealed;
// endregion: --- States

// Make the RequestBuilder generic over type U, which is the "url" type
// To enforce "method", make it generic over M as well:
#[derive(Default, Clone)]
pub struct RequestBuilder<U, M, S> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,  // (name, value)
    body: Option<String>,

    // We need PhantomData<S> to make use of S, otherwise S is unused and the code won't compile
    marker_seal: PhantomData<S>,
}

// Implement new() for the NoUrl type
impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

// For any not-sealed U and M, implement .sealed():
impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn seal(self) -> RequestBuilder<U, M, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }
}

// Move RequestBuilder::build into a specialised case, and remove the run-time check.
// Generic over S because we can build with a Sealed or NotSealed builder:
impl<S> RequestBuilder<Url, Method, S> {
    // build is now infallible, no need to return Result:
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,  // from Url(String), which is the specific generic type
            method: self.method.0,  // from Method(String)
            headers: self.headers,
            body: self.body,
        }
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {

    // Return type is now RequestBuilder<Url>
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }

    pub fn method(self, method: impl Into<String>) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
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

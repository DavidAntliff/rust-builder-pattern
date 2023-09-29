// https://www.youtube.com/watch?v=Z_3WOSiYYFY
pub mod request_builder_non_consuming;
pub mod request_builder_consuming;
pub mod request_builder_type_state;

// Re-export doesn't work since both RequestBuilders are called the same thing
// pub use crate::request::request_builder_non_consuming::RequestBuilder;
// pub use crate::request::request_builder_consuming::RequestBuilder;
// pub use crate::request::request_builder_type_state::RequestBuilder;

#[derive(Debug)]
pub struct Request {
    url: String,
    method :String,
    headers: Vec<(String, String)>,  // (name, value)
    body: Option<String>,
}

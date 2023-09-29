// https://www.youtube.com/watch?v=Z_3WOSiYYFY

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method :String,
    pub headers: Vec<(String, String)>,  // (name, value)
    pub body: Option<String>,
}

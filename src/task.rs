#[derive(Debug)]
pub struct Task {
    title: String,
    done: bool,
    desc: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            done: false,
            desc: None,
        }
    }
}

impl Task {
    pub fn new(title: impl Into<String>) -> Task {
        Task {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}



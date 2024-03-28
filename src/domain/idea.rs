#[derive(Clone)]
pub struct Idea {
    pub title: String,
    pub tagline: String,
}

impl Idea {
    pub fn new(title: &str, tagline: &str) -> Self {
        Self {
            title: title.to_string(),
            tagline: tagline.to_string(),
        }
    }
}

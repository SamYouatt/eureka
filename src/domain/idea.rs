use uuid::Uuid;

#[derive(Clone)]
pub struct Idea {
    pub id: Uuid,
    pub title: String,
    pub tagline: String,
}

impl Idea {
    pub fn new(title: &str, tagline: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            tagline: tagline.to_string(),
        }
    }
}

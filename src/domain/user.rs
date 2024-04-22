use uuid::Uuid;

#[derive(Debug)]
pub struct AppUser {
    pub email: String,
    pub id: Uuid,
}

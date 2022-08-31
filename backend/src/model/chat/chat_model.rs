use uuid::Uuid;

pub struct ChatMember {
    pub(super) author_id: Uuid,
    pub(super) first_name: String,
    pub(super) last_name: Option<String>,
}

pub struct Chat {
    pub id: i64,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

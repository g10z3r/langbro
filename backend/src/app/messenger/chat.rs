pub struct Chat {
    pub id: i64,
    pub type_lb: ChatType,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub enum ChatType {
    Private,
}

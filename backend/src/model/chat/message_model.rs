use super::chat_model::ChatMember;

pub struct Message {
    pub message_id: i64,
    pub from_id: Box<ChatMember>,
    pub forward_from: Option<Box<ChatMember>>,
    pub reply_to_message: Option<Box<Message>>,
    pub text: Option<String>,
    pub language_code: Option<String>,
    pub date: i64,
}

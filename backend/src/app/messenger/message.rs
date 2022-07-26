use mongodb::bson::oid::ObjectId;

use crate::app::account::Account;

use super::chat::Chat;

pub struct Message {
    /// Уникальный идентификатор сообщения внутри этого чата
    pub message_id: ObjectId,
    /// Отправитель сообщения
    pub from: Box<Account>,
    /// Дата отправки сообщения в формате Unix
    pub date: i64,
    /// Диалог, которому принадлежит сообщение
    pub chat: Box<Chat>,
    /// Отправитель исходного сообщения
    pub forward_from: Option<Box<Account>>,
    /// Дата отправки исходного сообщения в формате Unix
    pub forward_date: Option<i64>,
    /// Ответ на исходное сообщение
    pub reply_to_message: Option<Box<Message>>,
    /// Дата последнего редактирования сообщения в формате Unix
    pub edit_date: Option<i64>,
    /// Фактический текст сообщения UTF-8, 0-500 символов.
    pub text: Option<String>,
}

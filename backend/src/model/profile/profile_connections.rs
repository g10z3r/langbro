use async_graphql::Enum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Enum, Display, EnumString)]
pub enum Cefr {
    /// Элементарное владение
    A,
    /// Уровень выживания
    A1,
    /// Предпороговый уровень
    A2,
    /// Самодостаточное владение
    B,
    // Пороговый уровень
    B1,
    /// Пороговый продвинутый уровень
    B2,
    /// Свободное владение
    C,
    /// Уровень профессионального владения
    C1,
    /// Уровень владения в совершенстве
    C2,
}

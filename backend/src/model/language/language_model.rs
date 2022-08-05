use anyhow::Result;
use async_graphql::Enum;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Enum, Display, EnumString)]
pub enum CefrKind {
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Enum, Display, EnumString)]
pub enum Language {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,

    Basque,
    Belarusian,
    Bengali,
    Bokmal,
    Bosnian,
    Bulgarian,

    Catalan,
    Chinese,
    Croatian,
    Czech,

    Danish,
    Dutch,

    English,
    Esperanto,
    Estonian,

    Finnish,
    French,

    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,

    Hebrew,
    Hindi,
    Hungarian,

    Icelandic,
    Indonesian,
    Irish,
    Italian,

    Japanese,

    Kazakh,
    Korean,

    Latin,
    Latvian,
    Lithuanian,

    Macedonian,
    Malay,
    Maori,
    Marathi,
    Mongolian,

    Nynorsk,

    Persian,
    Polish,
    Portuguese,
    Punjabi,

    Romanian,
    Russian,

    Serbian,
    Shona,
    Slovak,
    Slovene,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,

    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Tsonga,
    Tswana,
    Turkish,

    Ukrainian,
    Urdu,

    Vietnamese,

    Welsh,

    Xhosa,

    Yoruba,

    Zulu,
}

impl Language {
    pub fn from_string_vec(vec: Vec<String>) -> Result<Vec<Language>> {
        let mut new_vec: Vec<Language> = Vec::new();

        for i in vec.iter() {
            new_vec.push(Language::from_str(i)?);
        }

        Ok(new_vec)
    }
}

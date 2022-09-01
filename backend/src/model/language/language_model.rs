use anyhow::Result;
use async_graphql::{Enum, Object};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

use super::language_mutation::StudiedInput;

/// Studied languages
#[derive(Serialize, Deserialize, Clone)]
pub struct Studied {
    pub cefr: CefrKind,
    pub lang: Language,
}

impl From<StudiedInput> for Studied {
    fn from(input: StudiedInput) -> Self {
        Studied {
            cefr: input.cefr,
            lang: input.lang,
        }
    }
}

impl Studied {
    pub(crate) fn new(cefr: CefrKind, lang: Language) -> Self {
        Self { cefr, lang }
    }
}

#[Object]
impl<'a> Studied {
    async fn cefr(&'a self) -> CefrKind {
        self.cefr
    }

    async fn lang(&'a self) -> Language {
        self.lang
    }
}

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

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Enum, Display, EnumString,
)]
pub enum Language {
    // A
    #[graphql(name = "Afrikaans")]
    Afrikaans,
    #[graphql(name = "Albanian")]
    Albanian,
    #[graphql(name = "Arabic")]
    Arabic,
    #[graphql(name = "Armenian")]
    Armenian,
    #[graphql(name = "Azerbaijani")]
    Azerbaijani,

    // B
    #[graphql(name = "Basque")]
    Basque,
    #[graphql(name = "Belarusian")]
    Belarusian,
    #[graphql(name = "Bengali")]
    Bengali,
    #[graphql(name = "Bokmal")]
    Bokmal,
    #[graphql(name = "Bosnian")]
    Bosnian,
    #[graphql(name = "Bulgarian")]
    Bulgarian,

    // C
    #[graphql(name = "Catalan")]
    Catalan,
    #[graphql(name = "Chinese")]
    Chinese,
    #[graphql(name = "Croatian")]
    Croatian,
    #[graphql(name = "Czech")]
    Czech,

    // D
    #[graphql(name = "Danish")]
    Danish,
    #[graphql(name = "Dutch")]
    Dutch,

    // E
    #[graphql(name = "English")]
    English,
    #[graphql(name = "Esperanto")]
    Esperanto,
    #[graphql(name = "Estonian")]
    Estonian,

    // F
    #[graphql(name = "Finnish")]
    Finnish,
    #[graphql(name = "French")]
    French,

    // G
    #[graphql(name = "Ganda")]
    Ganda,
    #[graphql(name = "Georgian")]
    Georgian,
    #[graphql(name = "German")]
    German,
    #[graphql(name = "Greek")]
    Greek,
    #[graphql(name = "Gujarati")]
    Gujarati,

    // H
    #[graphql(name = "Hebrew")]
    Hebrew,
    #[graphql(name = "Hindi")]
    Hindi,
    #[graphql(name = "Hungarian")]
    Hungarian,

    // I
    #[graphql(name = "Icelandic")]
    Icelandic,
    #[graphql(name = "Indonesian")]
    Indonesian,
    #[graphql(name = "Irish")]
    Irish,
    #[graphql(name = "Italian")]
    Italian,

    // J
    #[graphql(name = "Japanese")]
    Japanese,

    // K
    #[graphql(name = "Kazakh")]
    Kazakh,
    #[graphql(name = "Korean")]
    Korean,

    // L
    #[graphql(name = "Latin")]
    Latin,
    #[graphql(name = "Latvian")]
    Latvian,
    #[graphql(name = "Lithuanian")]
    Lithuanian,

    // M
    #[graphql(name = "Macedonian")]
    Macedonian,
    #[graphql(name = "Malay")]
    Malay,
    #[graphql(name = "Maori")]
    Maori,
    #[graphql(name = "Marathi")]
    Marathi,
    #[graphql(name = "Mongolian")]
    Mongolian,

    // N
    #[graphql(name = "Nynorsk")]
    Nynorsk,

    // P
    #[graphql(name = "Persian")]
    Persian,
    #[graphql(name = "Polish")]
    Polish,
    #[graphql(name = "Portuguese")]
    Portuguese,
    #[graphql(name = "Punjabi")]
    Punjabi,

    // R
    #[graphql(name = "Romanian")]
    Romanian,
    #[graphql(name = "Russian")]
    Russian,

    // S
    #[graphql(name = "Serbian")]
    Serbian,
    #[graphql(name = "Shona")]
    Shona,
    #[graphql(name = "Slovak")]
    Slovak,
    #[graphql(name = "Slovene")]
    Slovene,
    #[graphql(name = "Somali")]
    Somali,
    #[graphql(name = "Sotho")]
    Sotho,
    #[graphql(name = "Spanish")]
    Spanish,
    #[graphql(name = "Swahili")]
    Swahili,
    #[graphql(name = "Swedish")]
    Swedish,

    // T
    #[graphql(name = "Tagalog")]
    Tagalog,
    #[graphql(name = "Tamil")]
    Tamil,
    #[graphql(name = "Telugu")]
    Telugu,
    #[graphql(name = "Thai")]
    Thai,
    #[graphql(name = "Tsonga")]
    Tsonga,
    #[graphql(name = "Tswana")]
    Tswana,
    #[graphql(name = "Turkish")]
    Turkish,

    // U
    #[graphql(name = "Ukrainian")]
    Ukrainian,
    #[graphql(name = "Urdu")]
    Urdu,

    // V
    #[graphql(name = "Vietnamese")]
    Vietnamese,

    // W
    #[graphql(name = "Welsh")]
    Welsh,

    // W
    #[graphql(name = "Xhosa")]
    Xhosa,

    // Y
    #[graphql(name = "Yoruba")]
    Yoruba,

    // Z
    #[graphql(name = "Zulu")]
    Zulu,
}

impl From<std::string::String> for Language {
    fn from(input: std::string::String) -> Self {
        input.into()
    }
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

impl From<neo4rs::types::BoltType> for Language {
    fn from(input: neo4rs::types::BoltType) -> Self {
        match input {
            neo4rs::types::BoltType::String(v) => Language::from_str(&v.value).unwrap(),
            _ => todo!(),
        }
    }
}

use juniper::{GraphQLEnum, GraphQLInputObject};
use std::str::FromStr;

use super::error::AppError;
use crate::err_validation;

#[derive(Clone, Serialize, Deserialize, GraphQLInputObject)]
pub struct CefrInput {
    language: Language,
    cefr: CefrKind,
}

impl From<CefrInput> for CEFR {
    fn from(ci: CefrInput) -> Self {
        Self {
            language: ci.language,
            cefr: ci.cefr,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CEFR {
    language: Language,
    cefr: CefrKind,
}

#[juniper::graphql_object]
impl CEFR {
    pub fn language(&self) -> Language {
        self.language.clone()
    }

    pub fn cefr(&self) -> CefrKind {
        self.cefr.clone()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, GraphQLEnum)]
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

impl std::fmt::Display for CefrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, GraphQLEnum)]
pub enum Language {
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

    #[graphql(name = "Catalan")]
    Catalan,
    #[graphql(name = "Chinese")]
    Chinese,
    #[graphql(name = "Croatian")]
    Croatian,
    #[graphql(name = "Czech")]
    Czech,

    #[graphql(name = "Danish")]
    Danish,
    #[graphql(name = "Dutch")]
    Dutch,

    #[graphql(name = "English")]
    English,
    #[graphql(name = "Esperanto")]
    Esperanto,
    #[graphql(name = "Estonian")]
    Estonian,

    #[graphql(name = "Finnish")]
    Finnish,
    #[graphql(name = "French")]
    French,

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

    #[graphql(name = "Hebrew")]
    Hebrew,
    #[graphql(name = "Hindi")]
    Hindi,
    #[graphql(name = "Hungarian")]
    Hungarian,

    #[graphql(name = "Icelandic")]
    Icelandic,
    #[graphql(name = "Indonesian")]
    Indonesian,
    #[graphql(name = "Irish")]
    Irish,
    #[graphql(name = "Italian")]
    Italian,

    #[graphql(name = "Japanese")]
    Japanese,

    #[graphql(name = "Kazakh")]
    Kazakh,
    #[graphql(name = "Korean")]
    Korean,

    #[graphql(name = "Latin")]
    Latin,
    #[graphql(name = "Latvian")]
    Latvian,
    #[graphql(name = "Lithuanian")]
    Lithuanian,

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

    #[graphql(name = "Nynorsk")]
    Nynorsk,

    #[graphql(name = "Persian")]
    Persian,
    #[graphql(name = "Polish")]
    Polish,
    #[graphql(name = "Portuguese")]
    Portuguese,
    #[graphql(name = "Punjabi")]
    Punjabi,

    #[graphql(name = "Romanian")]
    Romanian,
    #[graphql(name = "Russian")]
    Russian,

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

    #[graphql(name = "Ukrainian")]
    Ukrainian,
    #[graphql(name = "Urdu")]
    Urdu,

    #[graphql(name = "Vietnamese")]
    Vietnamese,

    #[graphql(name = "Welsh")]
    Welsh,

    #[graphql(name = "Xhosa")]
    Xhosa,

    #[graphql(name = "Yoruba")]
    Yoruba,

    #[graphql(name = "Zulu")]
    Zulu,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Language {
    type Err = AppError;

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "Afrikaans" => Ok(Language::Afrikaans),
            "Albanian" => Ok(Language::Albanian),
            "Arabic" => Ok(Language::Arabic),
            "Armenian" => Ok(Language::Armenian),
            "Azerbaijani" => Ok(Language::Azerbaijani),

            "Basque" => Ok(Language::Basque),
            "Belarusian" => Ok(Language::Belarusian),
            "Bengali" => Ok(Language::Bengali),
            "Bokmal" => Ok(Language::Bokmal),
            "Bosnian" => Ok(Language::Bosnian),
            "Bulgarian" => Ok(Language::Bulgarian),

            "Catalan" => Ok(Language::Catalan),
            "Chinese" => Ok(Language::Chinese),
            "Croatian" => Ok(Language::Croatian),
            "Czech" => Ok(Language::Czech),

            "Danish" => Ok(Language::Danish),
            "Dutch" => Ok(Language::Dutch),

            "English" => Ok(Language::English),
            "Esperanto" => Ok(Language::Esperanto),
            "Estonian" => Ok(Language::Estonian),

            "Finnish" => Ok(Language::Finnish),
            "French" => Ok(Language::French),

            "Ganda" => Ok(Language::Ganda),
            "Georgian" => Ok(Language::Georgian),
            "German" => Ok(Language::German),
            "Greek" => Ok(Language::Greek),
            "Gujarati" => Ok(Language::Gujarati),

            "Hebrew" => Ok(Language::Hebrew),
            "Hindi" => Ok(Language::Hindi),
            "Hungarian" => Ok(Language::Hungarian),

            "Icelandic" => Ok(Language::Icelandic),
            "Indonesian" => Ok(Language::Indonesian),
            "Irish" => Ok(Language::Irish),
            "Italian" => Ok(Language::Italian),

            "Japanese" => Ok(Language::Japanese),

            "Kazakh" => Ok(Language::Kazakh),
            "Korean" => Ok(Language::Korean),

            "Latin" => Ok(Language::Latin),
            "Latvian" => Ok(Language::Latvian),
            "Lithuanian" => Ok(Language::Lithuanian),

            "Macedonian" => Ok(Language::Macedonian),
            "Malay" => Ok(Language::Malay),
            "Maori" => Ok(Language::Maori),
            "Marathi" => Ok(Language::Marathi),
            "Mongolian" => Ok(Language::Mongolian),

            "Nynorsk" => Ok(Language::Nynorsk),

            "Persian" => Ok(Language::Persian),
            "Polish" => Ok(Language::Polish),
            "Portuguese" => Ok(Language::Portuguese),
            "Punjabi" => Ok(Language::Punjabi),

            "Romanian" => Ok(Language::Romanian),
            "Russian" => Ok(Language::Russian),

            "Serbian" => Ok(Language::Serbian),
            "Shona" => Ok(Language::Shona),
            "Slovak" => Ok(Language::Slovak),
            "Slovene" => Ok(Language::Slovene),
            "Somali" => Ok(Language::Somali),
            "Sotho" => Ok(Language::Sotho),
            "Spanish" => Ok(Language::Spanish),
            "Swahili" => Ok(Language::Swahili),
            "Swedish" => Ok(Language::Swedish),

            "Tagalog" => Ok(Language::Tagalog),
            "Tamil" => Ok(Language::Tamil),
            "Telugu" => Ok(Language::Telugu),
            "Thai" => Ok(Language::Thai),
            "Tsonga" => Ok(Language::Tsonga),
            "Tswana" => Ok(Language::Tswana),
            "Turkish" => Ok(Language::Turkish),

            "Ukrainian" => Ok(Language::Ukrainian),
            "Urdu" => Ok(Language::Urdu),

            "Vietnamese" => Ok(Language::Vietnamese),

            "Welsh" => Ok(Language::Welsh),

            "Xhosa" => Ok(Language::Xhosa),

            "Yoruba" => Ok(Language::Yoruba),

            "Zulu" => Ok(Language::Zulu),

            _ => Err(err_validation!("Unknown value for language".to_string())),
        }
    }
}

use async_graphql::InputObject;

use super::language_model::{CefrKind, Language};

#[derive(Serialize, Deserialize, InputObject)]
pub(crate) struct StudLangInput {
    pub cefr: CefrKind,
    pub lang: Language,
}

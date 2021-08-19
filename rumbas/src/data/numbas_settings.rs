use crate::data::locale::SupportedLocale;
use crate::data::template::{Value, ValueType};
use crate::support::optional_overwrite::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//TODO: is locale still being used?
optional_overwrite! {
    pub struct NumbasSettings {
        locale: SupportedLocale,
        theme: String //TODO: check if valid theme? Or is numbas error ok?
    }
}

use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::Value;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct QuestionNavigation {
        /// Whether the student can regenerate the question
        /// Old name was `allow_regenerate`
        #[serde(alias = "allow_regenerate")]
        can_regenerate: RumbasBool,
        /// Whether the title page should be shown.
        /// Old name was `show_frontpage`
        #[serde(alias = "show_frontpage")]
        show_title_page: RumbasBool,
        /// Whether the student will be asked to confirm when leaving the exam.
        #[serde(alias = "prevent_leaving")]
        confirm_when_leaving: RumbasBool
    }
}

impl ToNumbas<numbas::question::navigation::Navigation> for QuestionNavigation {
    fn to_numbas(&self, locale: &str) -> numbas::question::navigation::Navigation {
        numbas::question::navigation::Navigation {
            allow_regenerate: self.can_regenerate.to_numbas(locale),
            show_frontpage: self.show_title_page.to_numbas(locale),
            confirm_when_leaving: Some(self.confirm_when_leaving.to_numbas(locale)),
        }
    }
}

impl ToRumbas<QuestionNavigation> for numbas::question::navigation::Navigation {
    fn to_rumbas(&self) -> QuestionNavigation {
        QuestionNavigation {
            can_regenerate: self.allow_regenerate.to_rumbas(),
            show_title_page: self.show_frontpage.to_rumbas(),
            confirm_when_leaving: self
                .confirm_when_leaving
                .unwrap_or(DEFAULTS.question_navigation_prevent_leaving)
                .to_rumbas(),
        }
    }
}

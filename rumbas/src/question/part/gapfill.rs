use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartGapFillInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    /// The Gap fill question part type
    pub struct QuestionPartGapFill {
        /// Whether the answers should be sorted
        sort_answers: bool,
        /// The gaps
        #[input(skip)]
        gaps: Vec<QuestionPart>
    }
}

impl ToNumbas<numbas::question::part::gapfill::QuestionPartGapFill> for QuestionPartGapFill {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::gapfill::QuestionPartGapFill {
        numbas::question::part::gapfill::QuestionPartGapFill {
            part_data: self.to_numbas(locale),
            sort_answers: self.sort_answers.to_numbas(locale),
            gaps: self.gaps.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartGapFill> for numbas::question::part::gapfill::QuestionPartGapFill {
    fn to_rumbas(&self) -> QuestionPartGapFill {
        create_question_part! {
            QuestionPartGapFill with &self.part_data => {

                sort_answers: self.sort_answers.to_rumbas(),

                gaps: self.gaps.to_rumbas()
            }
        }
    }
}

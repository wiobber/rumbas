use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStringInput;
use numbas::defaults::DEFAULTS;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Feedback {
        percentage_needed_to_pass: NoneableFloat, // if "none" (or 0) -> no percentage shown in frontpage, otherwise it is shown
        show_name_of_student: RumbasBool,
        /// Whether current marks are shown during exam or not (show_actual_mark in numbas)
        show_current_marks: RumbasBool,
        /// Whether the maximal mark for a question (or the total exam) is shown (show_total_mark of numbas)
        show_maximum_marks: RumbasBool,
        /// Whether answer feedback is shown (right or wrong etc)
        show_answer_state: RumbasBool,
        /// Whether the 'reveal answer' button is present
        allow_reveal_answer: RumbasBool,
        review: Review, // If none, everything is true???
        advice: TranslatableString,
        intro: TranslatableString,
        feedback_messages: FeedbackMessages
    }
}

impl ToNumbas<numbas::exam::feedback::Feedback> for Feedback {
    fn to_numbas(&self, locale: &str) -> numbas::exam::feedback::Feedback {
        numbas::exam::feedback::Feedback {
            show_actual_mark: self.show_current_marks.to_numbas(locale),
            show_total_mark: self.show_maximum_marks.to_numbas(locale),
            show_answer_state: self.show_answer_state.to_numbas(locale),
            allow_reveal_answer: self.allow_reveal_answer.to_numbas(locale),
            review: self.review.clone().map(|o| o.to_numbas(locale)),
            advice: self.advice.clone().map(|o| o.to_string(locale)).flatten(),
            intro: self.intro.clone().to_numbas(locale),
            feedback_messages: self.feedback_messages.to_numbas(locale),
        }
    }
}

impl ToRumbas<Feedback> for numbas::exam::exam::Exam {
    fn to_rumbas(&self) -> Feedback {
        let review: Option<_> = self.feedback.review.to_rumbas();
        Feedback {
            percentage_needed_to_pass: self.basic_settings.percentage_needed_to_pass.to_rumbas(),
            show_name_of_student: self
                .basic_settings
                .show_student_name
                .unwrap_or(DEFAULTS.basic_settings_show_student_name)
                .to_rumbas(),
            show_current_marks: self.feedback.show_actual_mark.to_rumbas(),
            show_maximum_marks: self.feedback.show_total_mark.to_rumbas(),
            show_answer_state: self.feedback.show_answer_state.to_rumbas(),
            allow_reveal_answer: self.feedback.allow_reveal_answer.to_rumbas(),
            review: Value::Normal(review.unwrap()), // TODO: fix this unwrap
            advice: self.feedback.advice.clone().unwrap_or_default().to_rumbas(),
            intro: self.feedback.intro.to_rumbas(),
            feedback_messages: self.feedback.feedback_messages.to_rumbas(),
        }
    }
}

optional_overwrite! {
    pub struct Review {
        /// Whether to show score in result overview page
        show_score: RumbasBool,
        /// Show feedback while reviewing
        show_feedback: RumbasBool,
        /// Show expected answer while reviewing
        show_expected_answer: RumbasBool,
        /// Show advice while reviewing
        show_advice: RumbasBool
    }
}

impl ToNumbas<numbas::exam::feedback::Review> for Review {
    fn to_numbas(&self, locale: &str) -> numbas::exam::feedback::Review {
        numbas::exam::feedback::Review {
            show_score: Some(self.show_score.to_numbas(locale)),
            show_feedback: Some(self.show_feedback.to_numbas(locale)),
            show_expected_answer: Some(self.show_expected_answer.to_numbas(locale)),
            show_advice: Some(self.show_advice.to_numbas(locale)),
        }
    }
}

impl ToRumbas<Review> for numbas::exam::feedback::Review {
    fn to_rumbas(&self) -> Review {
        Review {
            show_score: self
                .show_score
                .unwrap_or(DEFAULTS.feedback_review_show_score)
                .to_rumbas(),
            show_feedback: self
                .show_feedback
                .unwrap_or(DEFAULTS.feedback_review_show_feedback)
                .to_rumbas(),
            show_expected_answer: self
                .show_expected_answer
                .unwrap_or(DEFAULTS.feedback_review_show_expected_answer)
                .to_rumbas(),
            show_advice: self
                .show_advice
                .unwrap_or(DEFAULTS.feedback_review_show_advice)
                .to_rumbas(),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    pub message: String,   //TODO: inputstring or filestring?
    pub threshold: String, //TODO type
}
impl_optional_overwrite!(FeedbackMessage);

impl ToNumbas<numbas::exam::feedback::FeedbackMessage> for FeedbackMessage {
    fn to_numbas(&self, locale: &str) -> numbas::exam::feedback::FeedbackMessage {
        numbas::exam::feedback::FeedbackMessage {
            message: self.message.to_numbas(locale),
            threshold: self.threshold.to_numbas(locale),
        }
    }
}

impl ToRumbas<FeedbackMessage> for numbas::exam::feedback::FeedbackMessage {
    fn to_rumbas(&self) -> FeedbackMessage {
        FeedbackMessage {
            message: self.message.to_rumbas(),
            threshold: self.threshold.to_rumbas(),
        }
    }
}

pub type FeedbackMessagesInput = Vec<Value<FeedbackMessageInput>>;
pub type FeedbackMessages = Vec<FeedbackMessage>;

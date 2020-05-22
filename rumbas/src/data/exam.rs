use crate::data::optional_overwrite::OptionalOverwrite;
use serde::Deserialize;
use serde::Serialize;

optional_overwrite! {
    Exam,
    name: String,
    duration_in_seconds: usize,
    percentage_needed_to_pass: f64,
    show_names_of_question_groups: bool,
    show_name_of_student: bool,
    navigation: Navigation,
    timing: Timing,
    feedback: Feedback
}

optional_overwrite! {
    Navigation,
    allow_regenerate: bool,
    reverse: bool,
    browsing_enabled: bool,
    allow_steps: bool,
    show_frontpage: bool,
    show_results_page: ShowResultsPage,
    prevent_leaving: bool,
    on_leave: Action,
    start_password: String
}

impl Navigation {
    fn to_numbas(&self) -> numbas::exam::ExamNavigation {
        //TODO: check empty
        numbas::exam::ExamNavigation::new(
            self.allow_regenerate.unwrap(),
            self.reverse,
            self.browsing_enabled,
            self.allow_steps,
            self.show_frontpage.unwrap(),
            self.show_results_page.map(|s| s.to_numbas()),
            self.prevent_leaving,
            self.on_leave.clone().map(|s| s.to_numbas()),
            self.start_password.clone(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShowResultsPage {
    OnCompletion,
    Never,
}
impl_optional_overwrite!(ShowResultsPage);
impl ShowResultsPage {
    fn to_numbas(&self) -> numbas::exam::ExamShowResultsPage {
        match self {
            ShowResultsPage::OnCompletion => numbas::exam::ExamShowResultsPage::OnCompletion,
            ShowResultsPage::Never => numbas::exam::ExamShowResultsPage::Never,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum Action {
    None { message: String },
}
impl_optional_overwrite!(Action);
impl Action {
    fn to_numbas(&self) -> numbas::exam::ExamAction {
        match self {
            Action::None { message } => numbas::exam::ExamAction::None {
                message: message.to_string(),
            },
        }
    }
}

optional_overwrite! {
    Timing,
    allow_pause: bool,
    on_timeout: Action,
    timed_warning: Action
}

impl Timing {
    fn to_numbas(&self) -> numbas::exam::ExamTiming {
        //TODO: check empty
        numbas::exam::ExamTiming::new(
            self.allow_pause.unwrap(),
            self.on_timeout.clone().unwrap().to_numbas(),
            self.timed_warning.clone().unwrap().to_numbas(),
        )
    }
}

optional_overwrite! {
    Feedback,
    show_actual_mark: bool,
    show_total_mark: bool,
    show_answer_state: bool,
    allow_reveal_answer: bool,
    review: Review,
    advice: String,
    intro: String,
    feedback_messages: Vec<FeedbackMessage>
}

impl Feedback {
    //TODO: check empty
    fn to_numbas(&self) -> numbas::exam::ExamFeedback {
        numbas::exam::ExamFeedback::new(
            self.show_actual_mark.unwrap(),
            self.show_total_mark.unwrap(),
            self.show_answer_state.unwrap(),
            self.allow_reveal_answer.unwrap(),
            self.review.clone().map(|s| s.to_numbas()),
            self.advice.clone(),
            self.intro.clone().unwrap(),
            self.feedback_messages
                .clone()
                .unwrap()
                .iter()
                .map(|s| s.to_numbas())
                .collect(),
        )
    }
}

optional_overwrite! {
    Review,
    show_score: bool,
    show_feedback: bool,
    show_expected_answer: bool,
    show_advice: bool
}

impl Review {
    //TODO: check empty
    fn to_numbas(&self) -> numbas::exam::ExamReview {
        numbas::exam::ExamReview::new(
            self.show_score,
            self.show_feedback,
            self.show_expected_answer,
            self.show_advice,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FeedbackMessage {
    message: String,
    threshold: String, //TODO type
}

impl FeedbackMessage {
    fn to_numbas(&self) -> numbas::exam::ExamFeedbackMessage {
        numbas::exam::ExamFeedbackMessage::new(self.message.clone(), self.threshold.clone())
    }
}

impl Exam {
    pub fn to_numbas(&self) -> numbas::exam::Exam {
        //TODO: check for empty fields
        let basic_settings = numbas::exam::BasicExamSettings::new(
            self.name.clone().unwrap(),
            self.duration_in_seconds,
            self.percentage_needed_to_pass,
            self.show_names_of_question_groups,
            self.show_name_of_student,
        );

        //TODO
        let resources: Vec<[String; 2]> = Vec::new();

        //TODO
        let extensions: Vec<String> = Vec::new();

        //TODO
        let custom_part_types: Vec<numbas::exam::CustomPartType> = Vec::new();

        //TODO
        let navigation = self.navigation.clone().unwrap().to_numbas();

        //TODO
        let timing = self.timing.clone().unwrap().to_numbas();

        //TODO
        let feedback = self.feedback.clone().unwrap().to_numbas();

        //TODO
        let functions = None;

        //TODO
        let variables = None;

        //TODO
        let question_groups: Vec<numbas::exam::ExamQuestionGroup> = Vec::new();

        numbas::exam::Exam::new(
            basic_settings,
            resources,
            extensions,
            custom_part_types,
            navigation,
            timing,
            feedback,
            functions,
            variables,
            question_groups,
        )
    }
}

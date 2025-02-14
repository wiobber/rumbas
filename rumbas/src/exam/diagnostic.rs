use crate::exam::feedback::Feedback;
use crate::exam::locale::Locale;
use crate::exam::locale::SupportedLocale;
use crate::exam::navigation::DiagnosticNavigation;
use crate::exam::numbas_settings::NumbasSettings;
use crate::exam::question_group::QuestionFromTemplate;
use crate::exam::question_group::QuestionGroup;
use crate::exam::timing::Timing;
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::question::extension::Extensions;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::JMENotesTranslatableString;
use crate::support::translatable::TranslatableString;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "DiagnosticExamInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
/// A Diagnostic Exam
pub struct DiagnosticExam {
    /// All locales for which the exam should be generated
    pub locales: Vec<Locale>,
    /// The name of the exam
    pub name: TranslatableString,
    /// The navigation settings for this exam
    pub navigation: DiagnosticNavigation,
    /// The timing settings for this exam
    pub timing: Timing,
    /// The feedback settings for this exam
    pub feedback: Feedback,
    /// The questions groups for this exam
    pub question_groups: Vec<QuestionGroup>,
    /// The settings to set for numbas
    pub numbas_settings: NumbasSettings,
    /// The diagnostic data
    pub diagnostic: Diagnostic,
}

impl ToNumbas<numbas::exam::Exam> for DiagnosticExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        let basic_settings = self.to_numbas(locale);

        let navigation = self.navigation.to_numbas(locale);

        let timing = self.timing.to_numbas(locale);

        let feedback = self.feedback.to_numbas(locale);

        let question_groups: Vec<numbas::exam::question_group::QuestionGroup> =
            self.question_groups.to_numbas(locale);

        let resources: Vec<numbas::question::resource::Resource> = self
            .question_groups
            .iter()
            .flat_map(|qg| {
                qg.clone()
                    .questions
                    .into_iter()
                    .flat_map(|q| q.data.resources)
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale); // TODO: extract?

        let extensions: Vec<String> = self
            .question_groups
            .iter()
            .flat_map(|qg| {
                qg.clone().questions.into_iter().map(|q| q.data.extensions) // todo: extract?
            })
            .fold(Extensions::default(), Extensions::combine)
            .to_paths();

        let custom_part_types: Vec<numbas::question::custom_part_type::CustomPartType> = self
            .question_groups
            .iter()
            .flat_map(|qg| {
                qg.clone()
                    .questions
                    .into_iter()
                    .flat_map(|q| q.data.custom_part_types)
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .to_numbas(locale); // todo extract?

        let diagnostic = Some(self.diagnostic.to_numbas(locale));

        numbas::exam::Exam {
            basic_settings,
            resources,
            extensions,
            custom_part_types,
            navigation,
            timing,
            feedback,
            question_groups,
            diagnostic,
        }
    }
}

impl ToNumbas<numbas::exam::BasicExamSettings> for DiagnosticExam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::BasicExamSettings {
        numbas::exam::BasicExamSettings {
            name: self.name.to_numbas(locale),
            duration_in_seconds: self
                .timing
                .duration_in_seconds
                .to_numbas(locale)
                .unwrap_or(0),
            percentage_needed_to_pass: self.feedback.percentage_needed_to_pass.to_numbas(locale),
            show_question_group_names: self.navigation.shared_data.show_names_of_question_groups,
            show_student_name: self.feedback.clone().show_name_of_student,
            allow_printing: self.navigation.shared_data.allow_printing,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "DiagnosticInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
/// Information needed for a diagnostic test
pub struct Diagnostic {
    /// The script to use
    pub script: DiagnosticScript,
    /// The learning objectives,
    pub objectives: Vec<LearningObjective>,
    /// The learning topics
    pub topics: Vec<LearningTopic>,
}

impl ToNumbas<numbas::exam::diagnostic::Diagnostic> for Diagnostic {
    fn to_numbas(&self, locale: &str) -> numbas::exam::diagnostic::Diagnostic {
        numbas::exam::diagnostic::Diagnostic {
            knowledge_graph: self.to_numbas(locale),
            script: self.script.to_numbas(locale),
            custom_script: self.script.to_numbas(locale),
        }
    }
}

impl ToNumbas<numbas::exam::diagnostic::DiagnosticKnowledgeGraph> for Diagnostic {
    fn to_numbas(&self, locale: &str) -> numbas::exam::diagnostic::DiagnosticKnowledgeGraph {
        numbas::exam::diagnostic::DiagnosticKnowledgeGraph {
            topics: self.topics.to_numbas(locale),
            learning_objectives: self.objectives.to_numbas(locale),
        }
    }
}

impl ToRumbas<Diagnostic> for numbas::exam::diagnostic::Diagnostic {
    fn to_rumbas(&self) -> Diagnostic {
        Diagnostic {
            script: self.to_rumbas(),
            objectives: self.knowledge_graph.learning_objectives.to_rumbas(),
            topics: self.knowledge_graph.topics.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "DiagnosticScriptInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticScript {
    Mastery,
    Diagnosys,
    Custom(JMENotesTranslatableString),
}

impl ToNumbas<numbas::exam::diagnostic::DiagnosticScript> for DiagnosticScript {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::diagnostic::DiagnosticScript {
        match self {
            DiagnosticScript::Mastery => numbas::exam::diagnostic::DiagnosticScript::Mastery,
            DiagnosticScript::Custom(_) => numbas::exam::diagnostic::DiagnosticScript::Custom,
            DiagnosticScript::Diagnosys => numbas::exam::diagnostic::DiagnosticScript::Diagnosys,
        }
    }
}

impl ToNumbas<numbas::jme::JMENotesString> for DiagnosticScript {
    fn to_numbas(&self, locale: &str) -> numbas::jme::JMENotesString {
        match self {
            DiagnosticScript::Custom(s) => s.to_numbas(locale),
            DiagnosticScript::Diagnosys => Default::default(),
            DiagnosticScript::Mastery => Default::default(),
        }
    }
}

impl ToRumbas<DiagnosticScript> for numbas::exam::diagnostic::Diagnostic {
    fn to_rumbas(&self) -> DiagnosticScript {
        match self.script {
            numbas::exam::diagnostic::DiagnosticScript::Mastery => DiagnosticScript::Mastery,
            numbas::exam::diagnostic::DiagnosticScript::Diagnosys => DiagnosticScript::Diagnosys,
            numbas::exam::diagnostic::DiagnosticScript::Custom => {
                DiagnosticScript::Custom(self.custom_script.clone().into())
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "LearningObjectiveInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
/// A Learning Objective
pub struct LearningObjective {
    /// The name
    pub name: TranslatableString,
    /// A description
    pub description: TranslatableString,
}

impl ToNumbas<numbas::exam::diagnostic::DiagnosticKnowledgeGraphLearningObjective>
    for LearningObjective
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::exam::diagnostic::DiagnosticKnowledgeGraphLearningObjective {
        numbas::exam::diagnostic::DiagnosticKnowledgeGraphLearningObjective {
            name: self.name.to_numbas(locale),
            description: self.description.to_numbas(locale),
        }
    }
}

impl ToRumbas<LearningObjective>
    for numbas::exam::diagnostic::DiagnosticKnowledgeGraphLearningObjective
{
    fn to_rumbas(&self) -> LearningObjective {
        LearningObjective {
            name: self.name.to_rumbas(),
            description: self.description.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "LearningTopicInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
/// A learning Topic
pub struct LearningTopic {
    /// The name
    pub name: TranslatableString,
    /// A description
    pub description: TranslatableString,
    /// List of names of objectives
    pub objectives: Vec<TranslatableString>,
    /// List of names of topic on which this topic depends
    pub depends_on: Vec<TranslatableString>,
}

impl ToNumbas<numbas::exam::diagnostic::DiagnosticKnowledgeGraphTopic> for LearningTopic {
    fn to_numbas(&self, locale: &str) -> numbas::exam::diagnostic::DiagnosticKnowledgeGraphTopic {
        numbas::exam::diagnostic::DiagnosticKnowledgeGraphTopic {
            name: self.name.to_numbas(locale),
            description: self.description.to_numbas(locale),
            learning_objectives: self.objectives.to_numbas(locale),
            depends_on: self.depends_on.to_numbas(locale),
        }
    }
}

impl ToRumbas<LearningTopic> for numbas::exam::diagnostic::DiagnosticKnowledgeGraphTopic {
    fn to_rumbas(&self) -> LearningTopic {
        LearningTopic {
            name: self.name.to_rumbas(),
            description: self.description.to_rumbas(),
            objectives: self.learning_objectives.to_rumbas(),
            depends_on: self.depends_on.to_rumbas(),
        }
    }
}

/// Converts a diagnostic numbas exam to a DiagnosticExam and extracts questions and
/// custom_part_types
pub fn convert_diagnostic_numbas_exam(
    exam: numbas::exam::Exam,
) -> (
    DiagnosticExam,
    Vec<QuestionFromTemplate>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let question_groups: Vec<_> = exam.question_groups.to_rumbas();
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        DiagnosticExam {
            locales: vec![Locale {
                name: "en".to_string(),
                numbas_locale: SupportedLocale::EnGB,
            }], // todo: argument?
            name: exam.basic_settings.name.to_rumbas(),
            navigation: exam.to_rumbas(),
            timing: exam.to_rumbas(),
            feedback: exam.to_rumbas(),
            question_groups: question_groups.clone(),
            numbas_settings: NumbasSettings {
                theme: "default".to_string(),
            }, // todo: argument?
            diagnostic: exam.diagnostic.unwrap().to_rumbas(), // Always set for a diagnostic exam
        },
        question_groups
            .into_iter()
            .flat_map(|qg| qg.questions)
            .collect(),
        custom_part_types,
    )
}

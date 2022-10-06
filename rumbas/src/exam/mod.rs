//! Contains all the exam types

pub mod diagnostic;
pub mod feedback;
pub mod locale;
pub mod navigation;
pub mod normal;
pub mod numbas_settings;
pub mod question_group;
pub mod timing;

use crate::exam::diagnostic::convert_diagnostic_numbas_exam;
use crate::exam::diagnostic::DiagnosticExam;
use crate::exam::locale::Locale;
use crate::exam::normal::convert_normal_numbas_exam;
use crate::exam::normal::NormalExam;
use crate::exam::question_group::QuestionFromTemplate;
use crate::question::custom_part_type::CustomPartTypeDefinitionPath;
use crate::support::default::combine_exam_with_default_files;
use crate::support::file_manager::CACHE;
use crate::support::template::{TemplateFile, TemplateFileInputEnum};
use crate::support::to_numbas::ToNumbas;
use crate::support::yaml::YamlError;
use comparable::Comparable;
use rumbas_support::path::RumbasPath;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::path::Path;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "ExamInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Exam {
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}

impl ToNumbas<numbas::exam::Exam> for Exam {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Exam {
        match self {
            Exam::Normal(n) => n.to_numbas(locale),
            Exam::Diagnostic(n) => n.to_numbas(locale),
        }
    }
}

impl Exam {
    pub fn locales(&self) -> Vec<Locale> {
        match self {
            Exam::Normal(n) => n.locales.clone(),
            Exam::Diagnostic(n) => n.locales.clone(),
        }
    }

    pub fn numbas_settings(&self) -> crate::exam::numbas_settings::NumbasSettings {
        match self {
            Exam::Normal(n) => n.numbas_settings.clone(),
            Exam::Diagnostic(n) => n.numbas_settings.clone(),
        }
    }
}
impl ExamInput {
    pub fn from_file(file: &RumbasPath) -> Result<ExamInput, ParseError> {
        use ExamFileTypeInput::*;
        let input: std::result::Result<ExamFileTypeInput, _> =
            if file.in_main_folder(crate::EXAMS_FOLDER) {
                let yaml = CACHE
                    .read_file(FileToLoad {
                        file_path: file.clone(),
                        locale_dependant: false,
                    })
                    .and_then(|lf| match lf {
                        LoadedFile::Normal(n) => Some(n.content),
                        LoadedFile::Localized(_) => None,
                    })
                    .ok_or_else(|| ParseError::FileReadError(FileReadError(file.clone())))?;

                serde_yaml::from_str(&yaml)
                    .map_err(|e| ParseError::YamlError(YamlError::from(e, file.clone())))
            } else if file.in_main_folder(crate::QUESTIONS_FOLDER) {
                let mut data = BTreeMap::new();
                data.insert(
                    "question".to_string(),
                    serde_yaml::Value::String(
                        file.project()
                            .with_extension("")
                            .strip_prefix(crate::QUESTIONS_FOLDER)
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    )
                    .into(),
                );
                let t = TemplateFile {
                    relative_template_path: crate::QUESTION_PREVIEW_TEMPLATE_NAME.to_string(),
                    data,
                };
                Ok(Template(TemplateFileInputEnum::from_normal(t)))
            } else {
                Err(ParseError::InvalidPath(InvalidExamPathError(file.clone())))
            };
        input
            .map(|e| match e {
                Normal(e) => Ok(ExamInput::Normal(e)),
                Diagnostic(e) => Ok(ExamInput::Diagnostic(e)),
                Template(t_val) => {
                    let t = t_val.to_normal();
                    let template_file = file.keep_root(
                        Path::new(crate::EXAMS_FOLDER)
                            .join(format!("{}.yaml", t.relative_template_path))
                            .as_path(),
                    ); // TODO: check for missing fields.....

                    let template_yaml = CACHE
                        .read_file(FileToLoad {
                            file_path: template_file.clone(),
                            locale_dependant: false,
                        })
                        .and_then(|lf| match lf {
                            LoadedFile::Normal(n) => Some(n.content),
                            LoadedFile::Localized(_) => None,
                        })
                        .ok_or_else(|| {
                            ParseError::FileReadError(FileReadError(template_file.clone()))
                        })?;

                    let mut exam: ExamInput = serde_yaml::from_str(&template_yaml)
                        .map_err(|e| ParseError::YamlError(YamlError::from(e, file.clone())))?;
                    t.data.iter().for_each(|(k, v)| {
                        exam.insert_template_value(k, &v.0);
                    });
                    Ok(exam)
                }
            })
            .and_then(std::convert::identity) //flatten result is currently only possible in nightly
    }

    pub fn combine_with_defaults(&mut self, path: &RumbasPath) {
        combine_exam_with_default_files(path.clone(), self);
    }
    pub fn load_files(&mut self, path: &RumbasPath) {
        loop {
            let files_to_load = self.files_to_load(path);
            if files_to_load.is_empty() {
                break;
            }
            let loaded_files = CACHE.read_files(files_to_load);
            self.insert_loaded_files(path, &loaded_files);
        }
    }
}

#[derive(Debug, Display)]
pub enum ParseError {
    YamlError(YamlError),
    FileReadError(FileReadError),
    InvalidPath(InvalidExamPathError),
    RecursiveTemplates(RecursiveTemplatesError),
}

#[derive(Debug)]
pub struct RecursiveTemplatesError(pub RumbasPath, pub Vec<RumbasPath>);

impl Display for RecursiveTemplatesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid templates setup: the template recursion contains a loop: {} has already been loaded in {}",
            self.0.display(),
            self.1.iter().map(|e| e.display().to_string()).collect::<Vec<_>>().join(" "),
        )
    }
}

#[derive(Debug)]
pub struct InvalidExamPathError(RumbasPath);

impl Display for InvalidExamPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid compilation path: {} should start with {}/ or {}/",
            self.0.display(),
            crate::EXAMS_FOLDER,
            crate::QUESTIONS_FOLDER
        )
    }
}

#[derive(Debug)]
pub struct FileReadError(pub RumbasPath);

impl Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed reading file: {}", self.0.display(),)
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "ExamFileTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ExamFileType {
    Template(TemplateFile),
    Normal(NormalExam),
    Diagnostic(DiagnosticExam),
}

impl ExamFileTypeInput {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl ExamFileType {
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        ExamFileTypeInput::from_normal(self.to_owned()).to_yaml()
    }
}

/// Convert a numbas exam to rumbas data
/// Returns the name of the exam, the resulting exam (as ExamFileType)
/// and vectors of questions and custom part type definitions
pub fn convert_numbas_exam(
    exam: numbas::exam::Exam,
) -> (
    String,
    ExamFileType,
    Vec<QuestionFromTemplate>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let (name, exam, qgs, cpts) = match exam.navigation.navigation_mode {
        numbas::exam::navigation::NavigationMode::Diagnostic(ref _d) => {
            let (exam, qgs, cpts) = convert_diagnostic_numbas_exam(exam);
            (exam.name.clone(), ExamFileType::Diagnostic(exam), qgs, cpts)
        }
        _ => {
            let (exam, qgs, cpts) = convert_normal_numbas_exam(exam);
            (exam.name.clone(), ExamFileType::Normal(exam), qgs, cpts)
        }
    };
    (
        name.to_string("").expect("no locale needed"),
        exam,
        qgs,
        cpts,
    )
}

use crate::path::RumbasPath;
use crate::value::Value;
use crate::value::ValueType;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct FileToLoad {
    pub file_path: RumbasPath,
    pub locale_dependant: bool,
}

#[derive(Debug, Clone)]
pub enum LoadedFile {
    Normal(LoadedNormalFile),
    Localized(LoadedLocalizedFile),
}

#[derive(Debug, Clone)]
pub struct LoadedNormalFile {
    pub file_path: RumbasPath,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct LoadedLocalizedFile {
    pub file_path: RumbasPath,
    pub content: Option<String>,
    pub localized_content: HashMap<String, String>,
}

pub trait Input: Clone {
    type Normal;

    /// This method assumes that it is called by a function that is initially called from `to_normal_safe`
    fn to_normal(&self) -> Self::Normal;

    /// Method that safely convets the input type to the normal type
    fn to_normal_safe(&self) -> Result<Self::Normal, InputCheckResult> {
        let check = self.find_missing();
        if check.is_empty() {
            Ok(self.to_normal())
        } else {
            Err(check)
        }
    }

    /// Check the optional data
    fn find_missing(&self) -> InputCheckResult;

    fn from_normal(normal: Self::Normal) -> Self;

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value);

    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad>;

    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &HashMap<FileToLoad, LoadedFile>,
    );

    // On which files is this input dependant?
    fn dependencies(&self, main_file_path: &RumbasPath) -> HashSet<RumbasPath>;
}

pub trait InputInverse {
    type Input;
    type EnumInput;
}

impl<O: InputInverse> InputInverse for Vec<O> {
    type Input = Vec<ValueType<<O as InputInverse>::Input>>;
    type EnumInput = Self::Input;
}
impl<O: Input> Input for Vec<O> {
    type Normal = Vec<<O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter().map(|a| a.to_normal()).collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal.into_iter().map(<O as Input>::from_normal).collect()
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        for (i, item) in self.iter().enumerate() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for item in self.iter_mut() {
            item.insert_template_value(key, val);
        }
    }

    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        self.iter()
            .flat_map(|f| f.files_to_load(main_file_path))
            .collect()
    }

    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &HashMap<FileToLoad, LoadedFile>,
    ) {
        for item in self.iter_mut() {
            item.insert_loaded_files(main_file_path, files);
        }
    }

    fn dependencies(&self, main_file_path: &RumbasPath) -> HashSet<RumbasPath> {
        self.iter()
            .flat_map(|a| a.dependencies(main_file_path))
            .collect()
    }
}

impl<O: InputInverse> InputInverse for HashMap<String, O> {
    type Input = HashMap<String, ValueType<<O as InputInverse>::Input>>;
    type EnumInput = Self::Input;
}
impl<O: Input> Input for HashMap<String, O> {
    type Normal = HashMap<String, <O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter()
            .map(|(s, a)| (s.to_owned(), a.to_normal()))
            .collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal
            .into_iter()
            .map(|(s, a)| (s, <O as Input>::from_normal(a)))
            .collect()
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        for (key, item) in self.iter() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(key.to_owned());
            result.union(&previous_result)
        }
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_key, item) in self.iter_mut() {
            item.insert_template_value(key, val);
        }
    }

    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        self.iter()
            .flat_map(|(_k, f)| f.files_to_load(main_file_path))
            .collect()
    }

    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &HashMap<FileToLoad, LoadedFile>,
    ) {
        for (_key, item) in self.iter_mut() {
            item.insert_loaded_files(main_file_path, files);
        }
    }

    fn dependencies(&self, main_file_path: &RumbasPath) -> HashSet<RumbasPath> {
        self.iter()
            .flat_map(|(_k, v)| v.dependencies(main_file_path))
            .collect()
    }
}

impl<O: InputInverse> InputInverse for BTreeMap<String, O> {
    type Input = BTreeMap<String, ValueType<<O as InputInverse>::Input>>;
    type EnumInput = Self::Input;
}
impl<O: Input> Input for BTreeMap<String, O> {
    type Normal = BTreeMap<String, <O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter()
            .map(|(s, a)| (s.to_owned(), a.to_normal()))
            .collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal
            .into_iter()
            .map(|(s, a)| (s, <O as Input>::from_normal(a)))
            .collect()
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        for (key, item) in self.iter() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(key.to_owned());
            result.union(&previous_result)
        }
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_key, item) in self.iter_mut() {
            item.insert_template_value(key, val);
        }
    }

    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        self.iter()
            .flat_map(|(_k, f)| f.files_to_load(main_file_path))
            .collect()
    }

    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &HashMap<FileToLoad, LoadedFile>,
    ) {
        for (_key, item) in self.iter_mut() {
            item.insert_loaded_files(main_file_path, files);
        }
    }

    fn dependencies(&self, main_file_path: &RumbasPath) -> HashSet<RumbasPath> {
        self.iter()
            .flat_map(|(_k, v)| v.dependencies(main_file_path))
            .collect()
    }
}

impl<O: InputInverse> InputInverse for Box<O> {
    type Input = Box<<O as InputInverse>::Input>;
    type EnumInput = Box<<O as InputInverse>::EnumInput>;
}
impl<O: Input> Input for Box<O> {
    type Normal = Box<<O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        Box::new((**self).to_normal())
    }

    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        Box::new(Input::from_normal(*normal))
    }

    fn find_missing(&self) -> InputCheckResult {
        (**self).find_missing()
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        (**self).insert_template_value(key, val)
    }

    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        (**self).files_to_load(main_file_path)
    }

    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &HashMap<FileToLoad, LoadedFile>,
    ) {
        (**self).insert_loaded_files(main_file_path, files)
    }

    fn dependencies(&self, main_file_path: &RumbasPath) -> HashSet<RumbasPath> {
        (**self).dependencies(main_file_path)
    }
}

impl<A: InputInverse, B: InputInverse> InputInverse for (A, B) {
    type Input = (
        Value<<A as InputInverse>::Input>,
        Value<<B as InputInverse>::Input>,
    );
    type EnumInput = Self::Input; // TODO?
}
impl<A: Input, B: Input> Input for (A, B) {
    type Normal = (<A as Input>::Normal, <B as Input>::Normal);

    fn to_normal(&self) -> <Self as Input>::Normal {
        (self.0.to_normal(), self.1.to_normal())
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        (
            <A as Input>::from_normal(normal.0),
            <B as Input>::from_normal(normal.1),
        )
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        let i = 0;
        let mut previous_result = self.0.find_missing();
        previous_result.extend_path(i.to_string());
        result.union(&previous_result);
        let i = 1;
        let mut previous_result = self.1.find_missing();
        previous_result.extend_path(i.to_string());
        result.union(&previous_result);
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.0.insert_template_value(key, val);
        self.1.insert_template_value(key, val);
    }

    fn files_to_load(&self, main_file_path: &RumbasPath) -> Vec<FileToLoad> {
        self.0
            .files_to_load(main_file_path)
            .into_iter()
            .chain(self.1.files_to_load(main_file_path).into_iter())
            .collect()
    }

    fn insert_loaded_files(
        &mut self,
        main_file_path: &RumbasPath,
        files: &HashMap<FileToLoad, LoadedFile>,
    ) {
        self.0.insert_loaded_files(main_file_path, files);
        self.1.insert_loaded_files(main_file_path, files);
    }

    fn dependencies(&self, main_file_path: &RumbasPath) -> HashSet<RumbasPath> {
        let mut result = HashSet::new();
        let previous_result = self.0.dependencies(main_file_path);
        result.extend(previous_result);
        let previous_result = self.1.dependencies(main_file_path);
        result.extend(previous_result);
        result
    }
}

macro_rules! impl_input {
    ($($t: ty),*) => {
        $(
        impl InputInverse for $t {
            type Input = Self;
            type EnumInput = Self;
        }
        impl Input for $t {
            type Normal = Self;

            fn to_normal(&self) -> <Self as Input>::Normal {
                self.to_owned()
            }

            fn from_normal(normal: <Self as Input>::Normal) -> Self {
                normal
            }

            fn find_missing(&self) -> InputCheckResult {
                InputCheckResult::empty()
            }

            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {

            }

            fn files_to_load(&self, _main_file_path: &RumbasPath) -> Vec<FileToLoad> { Vec::new() }

            fn insert_loaded_files(&mut self, _main_file_path: &RumbasPath, _files: &HashMap<FileToLoad, LoadedFile>) {}

            fn dependencies(&self, _main_file_path: &RumbasPath) -> HashSet<RumbasPath> {
                HashSet::new()
            }
        }
        )*
    };
}

impl_input!(String);
impl_input!(f64, f32, [f64; 2]);
impl_input!(u128, u64, u32, u16, u8, usize);
impl_input!(i128, i64, i32, i16, i8, isize);
impl_input!(bool);

impl_input!(std::path::PathBuf);

impl_input!(numbas::jme::ContentAreaString);
impl_input!(numbas::jme::EmbracedJMEString);
impl_input!(numbas::jme::JMEString);
impl_input!(numbas::question::part::match_answers::MatchAnswersWithChoicesLayout);
impl_input!(numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType);
impl_input!(numbas::question::part::match_answers::MultipleChoiceWarningType);
impl_input!(numbas::question::part::pattern_match::PatternMatchMode);
impl_input!(numbas::support::answer_style::AnswerStyle);
impl_input!(numbas::question::function::FunctionType);
impl_input!(numbas::question::custom_part_type::CustomPartTypeSetting);
impl_input!(numbas::support::primitive::Number);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputCheckResult {
    // When adding a field, do also add it to is_empty
    missing_values: Vec<InputCheckMissingData>,
    invalid_yaml_values: Vec<InputCheckInvalidYamlData>,
    error_messages: Vec<String>,
}

impl InputCheckResult {
    pub fn from_missing(os: Option<String>) -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![InputCheckMissingData {
                path: InputCheckPath::with_last(os),
            }],
            invalid_yaml_values: vec![],
            error_messages: Vec::new(),
        }
    }
    pub fn from_invalid(v: &serde_yaml::Value, e: Option<serde_yaml::Error>) -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![],
            invalid_yaml_values: vec![InputCheckInvalidYamlData {
                path: InputCheckPath::without_last(),
                data: v.clone(),
                error: e.map(|e| e.to_string()),
            }],
            error_messages: Vec::new(),
        }
    }
    pub fn from_error_message(s: String) -> Self {
        Self {
            missing_values: Vec::new(),
            invalid_yaml_values: Vec::new(),
            error_messages: vec![s],
        }
    }
    pub fn empty() -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![],
            invalid_yaml_values: vec![],
            error_messages: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.missing_values.is_empty()
            && self.invalid_yaml_values.is_empty()
            && self.error_messages.is_empty()
    }
    pub fn extend_path(&mut self, s: String) {
        for missing_value in self.missing_values.iter_mut() {
            missing_value.path.add(s.clone());
        }
        for invalid_value in self.invalid_yaml_values.iter_mut() {
            invalid_value.path.add(s.clone());
        }
    }
    pub fn union(&mut self, other: &Self) {
        self.missing_values.extend(other.missing_values.clone());
        self.invalid_yaml_values
            .extend(other.invalid_yaml_values.clone());

        self.error_messages.extend(other.error_messages.clone());
    }
    pub fn missing_fields(&self) -> Vec<InputCheckMissingData> {
        self.missing_values.clone()
    }
    pub fn invalid_yaml_fields(&self) -> Vec<InputCheckInvalidYamlData> {
        self.invalid_yaml_values.clone()
    }
    pub fn error_messages(&self) -> Vec<String> {
        self.error_messages.clone()
    }
}

impl InputCheckResult {
    pub fn log(&self, path: &RumbasPath) {
        log::error!("Error when processing {}.", path.display());
        if !self.missing_values.is_empty() {
            log::error!("Found {} missing fields:", self.missing_values.len());
            for (idx, error) in self.missing_values.iter().enumerate() {
                log::error!("{}\t{}", idx + 1, error.to_string());
            }
        }
        if !self.invalid_yaml_values.is_empty() {
            log::error!("Found {} invalid fields:", self.invalid_yaml_values.len());
            for (idx, error) in self.invalid_yaml_values.iter().enumerate() {
                log::error!("{}\t{}", idx + 1, error.to_string());
            }
        }
        if !self.error_messages.is_empty() {
            log::error!("Found {} error messages:", self.error_messages.len());
            for (idx, error) in self.error_messages.iter().enumerate() {
                log::error!("{}\t{}", idx + 1, error);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputCheckPath {
    parts: Vec<String>,
    last_part: Option<String>,
}

impl InputCheckPath {
    pub fn with_last(os: Option<String>) -> Self {
        InputCheckPath {
            parts: vec![],
            last_part: os,
        }
    }
    pub fn without_last() -> Self {
        Self::with_last(None)
    }
    pub fn add(&mut self, s: String) {
        self.parts.insert(0, s)
    }
}

impl std::fmt::Display for InputCheckPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let base = self.parts.join(".");
        write!(
            f,
            "{}",
            if let Some(ref e) = self.last_part {
                format!("{}.{}", base, e)
            } else {
                base
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputCheckMissingData {
    path: InputCheckPath,
}

impl std::fmt::Display for InputCheckMissingData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputCheckInvalidYamlData {
    path: InputCheckPath,
    data: serde_yaml::Value,
    error: Option<String>,
}

impl std::fmt::Display for InputCheckInvalidYamlData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p = self.path.to_string();
        write!(
            f,
            "{}{}",
            if let Ok(s) = serde_yaml::to_string(&self.data) {
                format!("{}\n With yaml:\n{}", p, s)
            } else {
                p
            },
            self.error
                .as_ref()
                .map(|s| format!("With error: {}", s))
                .unwrap_or_else(String::new)
        )
    }
}

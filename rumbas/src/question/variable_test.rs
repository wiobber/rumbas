use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use comparable::Comparable;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "VariablesTestInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct VariablesTest {
    pub condition: JMEString,
    pub max_runs: usize,
}

impl ToNumbas<numbas::question::QuestionVariablesTest> for VariablesTest {
    fn to_numbas(&self, locale: &str) -> numbas::question::QuestionVariablesTest {
        numbas::question::QuestionVariablesTest {
            condition: self.condition.to_numbas(locale),
            max_runs: self.max_runs.to_numbas(locale),
        }
    }
}

impl ToRumbas<VariablesTest> for numbas::question::QuestionVariablesTest {
    fn to_rumbas(&self) -> VariablesTest {
        VariablesTest {
            condition: self.condition.to_rumbas(),
            max_runs: self.max_runs.0.to_rumbas(),
        }
    }
}

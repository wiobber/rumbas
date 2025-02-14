use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! extensions {
    (
        $(
            $(#[$inner:meta])*
            $name: ident: $path: literal
        ),+
    ) => {
            #[derive(Input, Overwrite, RumbasCheck, Examples)]
            #[input(name = "ExtensionsInput")]
            #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
            /// Specify which extensions should be enabled
            pub struct Extensions {
                $(
                    $(
                        #[$inner]
                    )*
                    pub $name: bool
                ),*
            }

        impl ToNumbas<Vec<String>> for Extensions {
            fn to_numbas(&self, locale: &str) -> Vec<String> {
                let mut extensions = Vec::new();
                $(
                    if self.$name.to_numbas(locale) {
                        extensions.push($path.to_string()); //TODO: Enum in numbas crate?
                    }
                )*
                extensions
            }
        }

        impl Extensions {
            pub fn from(v: &[String]) -> Self {
                Extensions {
                    $(
                        $name: v.contains(&$path.to_string()).to_rumbas()
                    ),*
                }
            }

            pub fn combine(e: Extensions, f: Extensions) -> Extensions {
                Extensions {
                    $(
                    $name: e.$name || f.$name
                    ),*
                }
            }

            pub fn to_paths(&self) -> Vec<String> {
                let numbas_path = std::env::var(crate::NUMBAS_FOLDER_ENV)
                    .expect(&format!("{} to be set", crate::NUMBAS_FOLDER_ENV)[..]);
                let mut paths = Vec::new();
                $(
                    if self.$name {
                        paths.push($path);
                    }
                )*
                paths
                    .into_iter()
                    .map(|s| format!("{}/extensions/{}", numbas_path, s))
                    .collect()
            }
        }

        impl Default for Extensions {
            fn default() -> Extensions {
                Extensions {
                    $(
                        $name: false
                    ),*
                }
            }
        }

    }
}

//fixme vis.js extension? (where to find?)
extensions! {
        chemistry: "chemistry",
        download_text_file: "download-text-file",
        eukleides: "eukleides",
        geogebra: "geogebra",
        graphs: "graphs",
        jsx_graph: "jsxgraph",
        linear_algebra: "linear-algebra",
        linear_codes: "codewords",
        optimisation: "optimisation",
        permutations: "permutations",
        polynomials: "polynomials",
        quantities: "quantities",
        random_person: "random_person",
        stats: "stats",
        sqlite: "sqlite",
        text: "text",
        written_number: "written-number"
}

impl ToRumbas<Extensions> for Vec<String> {
    fn to_rumbas(&self) -> Extensions {
        Extensions::from(self)
    }
}

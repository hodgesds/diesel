use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use toml;

use super::{find_project_root, handle_error};
use print_schema;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub print_schema: PrintSchema,
}

impl Config {
    pub fn file_path(matches: &ArgMatches) -> PathBuf {
        matches
            .value_of("CONFIG_FILE")
            .map(PathBuf::from)
            .or_else(|| env::var_os("DIESEL_CONFIG_FILE").map(PathBuf::from))
            .unwrap_or_else(|| {
                find_project_root()
                    .unwrap_or_else(handle_error)
                    .join("diesel.toml")
            })
    }

    pub fn read(matches: &ArgMatches) -> Result<Self, Box<Error>> {
        let path = Self::file_path(matches);
        let mut bytes = Vec::new();
        fs::File::open(path)?.read_to_end(&mut bytes)?;
        toml::from_slice(&bytes).map_err(Into::into)
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrintSchema {
    #[serde(default)]
    pub file: Option<PathBuf>,
    #[serde(default)]
    pub with_docs: bool,
    #[serde(default)]
    pub filter: print_schema::Filtering,
    #[serde(default)]
    pub schema: Option<String>,
}

impl PrintSchema {
    pub fn schema_name(&self) -> Option<&str> {
        self.schema.as_ref().map(|s| &**s)
    }
}

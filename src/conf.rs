/*  Copyright (c) 2021 Jeremy Carter <jeremy@jeremycarter.ca>

    All uses of this project in part or in whole are governed
    by the terms of the license contained in the file titled
    "LICENSE" that's distributed along with the project, which
    can be found in the top-level directory of this project.

    If you don't agree to follow those terms or you won't
    follow them, you are not allowed to use this project or
    anything that's made with parts of it at all. The project
    is also	depending on some third-party technologies, and
    some of those are governed by their own separate licenses,
    so furthermore, whenever legally possible, all license
    terms from all of the different technologies apply, with
    this project's license terms taking first priority.

    NOTE: This file includes some possibly modified example
    code from:
    https://github.com/gfx-rs/gfx/blob/master/examples/quad/main.rs
*/

use crate::{constant::*, file, NovResultError};

use serde::{Deserialize, Serialize};

use std::fmt::Display;
use std::fs;

pub trait ConfKind {
    fn new() -> Self;
    fn default() -> Self;
    fn to_yaml(&self) -> String;
    fn from_yaml(s: String) -> Self;
    fn is_empty(&self) -> bool;
    fn set_file_read_default(&mut self, filename: &str);
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NovConf {
    pub file_read_directories: Vec<String>,
    pub file_read_default: String,
}

impl NovConf {}

/** Load a config file.

`filename` is the config file to load. If it's `None`,
a default path will be tried. If a config file isn't
found at the path, a new one will be created and filled
with initial default values.

`returns` a struct containing the config file's values. */
pub fn load<T: ConfKind>(filename: Option<&str>) -> Result<T, NovResultError> {
    let filename2 = filename.unwrap_or(CONF_FILE_DEFAULT);

    let mut file_content = Vec::<u8>::new();
    let new_file = file::read(
        &mut file_content,
        Some(filename2),
        Some(
            CONF_FILE_DEFAULT_PREFIXES
                .to_vec()
                .iter()
                .map(|res| res.to_string())
                .collect(),
        ),
    )
    .map_or_else(
        |_err| {
            println!(
                "Config file not found. Creating a new one: {}",
                CONF_FILE_DEFAULT
            );

            true
        },
        |res| {
            let (filename, _prefixes) = res;
            println!("config file found: {}", filename);

            false
        },
    );

    if new_file {
        let c = T::default();
        fs::write(CONF_FILE_DEFAULT, c.to_yaml()).unwrap();
        println!("config loaded:\n{}", &c.to_yaml());

        Ok(c)
    } else {
        let c = T::from_yaml(std::str::from_utf8(&file_content).unwrap().to_string());

        if c.is_empty() {
            let err = "There is some problem in the conf.yaml config file. \
If you can't solve it, you can delete the file and a new one will be \
created with default values."
                .to_string();
            eprintln!("error: {}", err);

            return Err((err, 8));
        }

        println!("config loaded:\n{}", &c.to_yaml());

        Ok(c)
    }
}

impl ConfKind for NovConf {
    fn new() -> Self {
        Self {
            file_read_directories: vec![],
            file_read_default: "".to_string(),
        }
    }

    fn default() -> Self {
        Self {
            file_read_directories: GET_PATH_PROJECT_FILE_PREFIXES
                .to_vec()
                .iter()
                .map(|val| val.to_string())
                .collect(),
            file_read_default: GET_PATH_PROJECT_FILENAME.to_string(),
        }
    }

    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }

    fn from_yaml(s: String) -> Self {
        serde_yaml::from_str(&s).unwrap_or_else(|err| {
            eprintln!("error: {}", err);
            Self::new()
        })
    }

    fn is_empty(&self) -> bool {
        *self == NovConf::new()
    }

    fn set_file_read_default(&mut self, filename: &str) {
        self.file_read_default = filename.to_string();
    }
}

impl Display for NovConf {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", &serde_json::to_string_pretty(&self).unwrap())
    }
}

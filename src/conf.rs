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

use crate::constant::*;

use serde::{Deserialize, Serialize};

pub trait ConfKind {
    fn new() -> Self;
    fn default() -> Self;
}

pub fn load<T: ConfKind>(filename: Option<&str>) -> T {
    let filename2 = filename.map_or_else(
        || {
            println!("loading default config values.");
            ""
        },
        |res| {
            println!("loading config file: {}", res);
            res
        },
    );

    if filename2 == "" {
        T::default()
    } else {
        T::new()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf<'a> {
    pub file_read_default: &'a str,
    pub file_path_prefixes: Vec<&'a str>,
}

impl<'a> Conf<'a> {}

impl<'a> ConfKind for Conf<'a> {
    fn new() -> Self {
        Self {
            file_read_default: "",
            file_path_prefixes: vec![],
        }
    }

    fn default() -> Self {
        Self {
            file_read_default: GET_PATH_PROJECT_FILENAME,
            file_path_prefixes: GET_PATH_PROJECT_FILE_PREFIXES.to_vec(),
        }
    }
}

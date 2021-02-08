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

pub const GET_PATH_DEFAULT_FILE_PREFIXES: [&str; 1] = [""];

pub const CONF_FILE_DEFAULT_PREFIXES: [&str; 6] = [
    GET_PATH_DEFAULT_FILE_PREFIXES[0],
    "../",
    "nov/",
    "../nov/",
    "libnov/",
    "../libnov/",
];

pub const CONF_FILE_DEFAULT: &str = "conf.yaml";

/// Search these directories when calling `file::get_path()`.
pub const GET_PATH_PROJECT_FILE_PREFIXES: [&str; 7] = [
    GET_PATH_DEFAULT_FILE_PREFIXES[0],
    "../",
    "data/",
    "../libnov/",
    "libnov/",
    "../libnov/data/",
    "libnov/data/",
];

pub const GET_PATH_PROJECT_FILENAME: &str = "ts-ellipsis-cover-m.png";

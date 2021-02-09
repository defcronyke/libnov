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

use crate::{constant::*, result::*};
use std::{fs, path};

pub fn get_path(
    src: Option<&str>,
    src_prefixes: Option<Vec<String>>,
) -> Result<(String, Vec<String>), NovResultError> {
    let filename_prefixes: Vec<String> = src_prefixes
        .unwrap_or(
            GET_PATH_DEFAULT_FILE_PREFIXES
                .to_vec()
                .iter()
                .map(|res| res.to_string())
                .collect(),
        )
        .into_iter()
        .map(|res| path::Path::new(&res).to_str().unwrap().to_string())
        .collect();

    let filename = src.map_or(GET_PATH_PROJECT_FILENAME.to_string(), |res| res.to_string());
    let mut filename2 = path::PathBuf::new();

    if filename.chars().count() == 0 {
        let err = "No filename specified.";

        eprintln!("error: {}", err);

        // TODO: Use a meaningful `NovResultErrorCode` here.
        return Err((err.to_string(), 1));
    }

    let mut filename_prefixes2 = GET_PATH_DEFAULT_FILE_PREFIXES
        .to_vec()
        .iter()
        .map(|res| res.to_string())
        .collect();

    let first_char: &str = &filename.chars().nth(0).unwrap().to_string();
    let mut second_char = "".to_string();
    let mut third_char = "".to_string();

    if filename.chars().count() > 3 {
        second_char = filename.chars().nth(1).unwrap().to_string();
        third_char = filename.chars().nth(2).unwrap().to_string();
    }

    // Handle absolute paths on Linux/UNIX, macOS, and Windows.
    if (filename.chars().count() < 2 || first_char != "/")
        && (filename.chars().count() < 4
            || (second_char.as_str() != ":" && third_char.as_str() != "\\"))
    {
        filename_prefixes2 = filename_prefixes.clone();
    }

    println!(
        "will check if file exists: {}
looking for file in the following directories: {:?}",
        &filename, filename_prefixes2
    );

    let mut file_found = false;

    for filename_prefix in filename_prefixes2.clone() {
        filename2 = path::Path::new(&filename_prefix).join(&filename);

        if filename2.exists() {
            println!("found file in directory: {:?}", &filename_prefix);
            file_found = true;
            break;
        }
    }

    if !file_found {
        let err = format!("File not found: {}", &filename);

        eprintln!("error: {}", &err);

        return Err((err, 2));
    }

    println!("found file: {}", filename2.display());

    Ok((filename2.to_str().unwrap().to_string(), filename_prefixes2))
}

pub fn read(
    dst: &mut Vec<u8>,
    src: Option<&str>,
    src_prefixes: Option<Vec<String>>,
) -> Result<(String, Vec<String>), NovResultError> {
    let (filename, filename_prefixes) = get_path(src, src_prefixes)?;

    *dst = fs::read(&filename).map_or_else(
        |err| {
            let err2 = format!("Failed reading file: {}: {}", &filename, err);
            eprintln!("error: {}", err2);
            // TODO: Use a meaningful error code here.
            Err((err2.to_string(), 6))
        },
        |res| {
            println!("successfully read file: {}", &filename);
            Ok(res)
        },
    )?;

    if dst.is_empty() {
        let err = format!("Empty result after trying to read file: {}", &filename);

        eprintln!("error: {}", &err);

        return Err((err, 3));
    }

    Ok((filename, filename_prefixes))
}

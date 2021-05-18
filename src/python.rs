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
*/

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn init() -> Result<(), ()> {
    println!("\n[ OPTIONAL FEATURE ENABLED ]: python\n");

    println!("[ python init ]: begin");

    let res = Python::with_gil(|py| {
        println!("[ python init global lock ]: begin\n");

        let res = _init(py).map_err(|err| {
            println!("\n[ python init error ]: begin\n");

            err.print_and_set_sys_last_vars(py);

            println!("\n[ python init error ]: end\n");
        });

        println!("[ python init global lock ]: end");

        res
    });

    println!("[ python init ]: end\n");

    res
}

fn _init(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;

    let version: String = sys.get("version")?.extract()?;

    let locals = [("os", py.import("os")?)].into_py_dict(py);

    let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";

    let user: String = py.eval(code, None, Some(&locals))?.extract()?;

    println!("[ python init ]: Hi {}, I'm Python {}\n", user, version);

    Ok(())
}

pub fn run(code: &str, globals: Vec<&str>, locals: Vec<&str>) -> Result<(), ()> {
    println!("[ python run ]: begin");

    let res = Python::with_gil(|py| {
        println!("[ python run global lock ]: begin\n");

        let res = _run(py, code, globals, locals).map_err(|err| {
            err.print_and_set_sys_last_vars(py);
        });

        println!("[ python run global lock ]: end");

        res
    });

    println!("[ python run ]: end\n");

    res
}

fn _run(py: Python, code: &str, globals: Vec<&str>, locals: Vec<&str>) -> PyResult<()> {
    println!("[ python run globals ]: {:?}\n", globals);

    println!("[ python run locals ]: {:?}\n", locals);

    let mut globals_py = vec![];
    let mut locals_py = vec![];

    for g in globals {
        globals_py.push((g, py.import(g)?));
    }

    for l in locals {
        locals_py.push((l, py.import(l)?));
    }

    println!("[ python run globals loaded ]: {:?}\n", globals_py);
    println!("[ python run locals loaded ]: {:?}\n", locals_py);

    // println!("[ python run code ]: begin\n");

    // println!("[ ---------------- ]\n");

    // println!("{}", code);

    // println!("[ ---------------- ]\n");

    // println!("[ python run code ]: end\n");

    let globals_py_dict = globals_py.clone().into_py_dict(py);
    let locals_py_dict = locals_py.clone().into_py_dict(py);

    let globals_py_option = {
        if globals_py.clone().len() > 0 {
            Some(globals_py_dict)
        } else {
            None
        }
    };

    let locals_py_option = {
        if locals_py.clone().len() > 0 {
            Some(locals_py_dict)
        } else {
            None
        }
    };

    println!("[ python run code output ]: begin\n");

    println!("[ ---------------- ]\n");

    let _res = py
        .run(code, globals_py_option, locals_py_option)
        .map_err(|err| {
            println!("{:?}\n", err);
            err.print(py);

            err
        });

    println!("\n[ ---------------- ]\n");

    println!("[ python run code output ]: end\n");

    Ok(())
}

pub fn run_file(filepath: &str, globals: Vec<&str>, locals: Vec<&str>) -> Result<(), ()> {
    let mut cwd = env::current_dir().unwrap_or_default();
    let mut cwd2 = env::current_dir().unwrap_or_default();

    let mut _filepath = {
        if filepath == "" {
            cwd.push("../libnov/data/src/main.py");
            cwd.to_str().unwrap()
        } else {
            filepath
        }
    };

    println!("[ python run file ]: begin");
    println!("[ python run file open ]: begin");

    println!("[ python run file opening ]: {}", _filepath);

    let mut file = File::open(_filepath)
        .map_or_else(
            |_err| {
                cwd2 = env::current_dir().unwrap_or_default();
                cwd2.push("data/src/main.py");
                _filepath = cwd2.to_str().unwrap();
                File::open(_filepath)
            },
            |res| Ok(res),
        )
        .expect("\n[ python run file open error ]: unable to open file\n");

    println!("[ python run file open ]: end");

    let mut contents = String::new();

    println!("[ python run file read ]\n");

    file.read_to_string(&mut contents)
        .expect("\n[ python run file read error ]: unable to read file\n");

    let res = run(&contents.as_str(), globals, locals);

    println!("[ python run file ]: end\n");

    res
}

pub fn eval(code: &str, globals: Vec<&str>, locals: Vec<&str>) -> Result<(), ()> {
    println!("[ python eval ]: begin");

    let res = Python::with_gil(|py| {
        println!("[ python eval global lock ]: begin\n");

        let res = _eval(py, code, globals, locals).map_err(|err| {
            err.print_and_set_sys_last_vars(py);
        });

        println!("[ python eval global lock ]: end");

        res
    });

    println!("[ python eval ]: end\n");

    res
}

fn _eval(py: Python, code: &str, globals: Vec<&str>, locals: Vec<&str>) -> PyResult<()> {
    println!("[ python eval locals ]: {:?}\n", locals);

    let mut globals_py = vec![];
    let mut locals_py = vec![];

    for g in globals {
        globals_py.push((g, py.import(g)?));
    }

    for l in locals {
        locals_py.push((l, py.import(l)?));
    }

    println!("[ python eval globals loaded ]: {:?}\n", globals_py);
    println!("[ python eval locals loaded ]: {:?}\n", locals_py);

    println!("[ python eval code ]: begin\n");

    println!("[ ---------------- ]\n");

    println!("{}", code);

    println!("[ ---------------- ]\n");

    println!("[ python eval code ]: end\n");

    let globals_py_dict = globals_py.clone().into_py_dict(py);
    let locals_py_dict = locals_py.clone().into_py_dict(py);

    println!("[ python eval code output ]: begin\n");

    println!("[ ---------------- ]\n");

    let globals_py_option = {
        if globals_py.clone().len() > 0 {
            Some(globals_py_dict)
        } else {
            None
        }
    };

    let locals_py_option = {
        if locals_py.clone().len() > 0 {
            Some(locals_py_dict)
        } else {
            None
        }
    };

    let _res = py.eval(code, globals_py_option, locals_py_option).unwrap();

    println!("\n[ ---------------- ]\n");

    println!("[ python eval code output ]: end\n");

    Ok(())
}

pub fn eval_file(filepath: &str, globals: Vec<&str>, locals: Vec<&str>) -> Result<(), ()> {
    let mut cwd = env::current_dir().unwrap_or_default();

    let _filepath = {
        if filepath == "" {
            cwd.push("../libnov/data/src/main.py");
            cwd.to_str().unwrap()
        } else {
            filepath
        }
    };

    println!("[ python eval file ]: begin");
    println!("[ python eval file open ]: begin");

    println!("[ python eval file opening ]: {}", _filepath);

    let mut file =
        File::open(_filepath).expect("\n[ python eval file open error ]: unable to open file\n");

    println!("[ python eval file open ]: end");

    let mut contents = String::new();

    println!("[ python eval file read ]\n");

    file.read_to_string(&mut contents)
        .expect("\n[ python eval file read error ]: unable to read file\n");

    let res = eval(&contents.as_str(), globals, locals);

    println!("[ python eval file ]: end\n");

    res
}

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

pub fn init() -> Result<(), ()> {
    Python::with_gil(|py| {
        main_(py).map_err(|e| {
            // We can't display Python exceptions via std::fmt::Display,
            // so print the error here manually.
            e.print_and_set_sys_last_vars(py);
        })
    })
}

fn main_(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let version: String = sys.get("version")?.extract()?;
    let locals = [("os", py.import("os")?)].into_py_dict(py);
    let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
    let user: String = py.eval(code, None, Some(&locals))?.extract()?;
    println!("Hello {}, I'm Python {}", user, version);
    Ok(())
}

// use cpython::{PyDict, PyResult, Python};

// pub fn init() {
//     let gil = Python::acquire_gil();
//     hello(gil.python()).unwrap();
// }

// fn hello(py: Python) -> PyResult<()> {
//     let sys = py.import("sys")?;
//     let version: String = sys.get(py, "version")?.extract(py)?;

//     let locals = PyDict::new(py);
//     locals.set_item(py, "os", py.import("os")?)?;
//     let user: String = py
//         .eval(
//             "os.getenv('USER') or os.getenv('USERNAME')",
//             None,
//             Some(&locals),
//         )?
//         .extract(py)?;

//     println!("Hello {}, I'm Python {}", user, version);
//     Ok(())
// }

// use pyembed::*;

// pub fn create_interpreter<'python, 'interpreter, 'resources>(
// ) -> Result<pyembed::MainPythonInterpreter<'python, 'interpreter, 'resources>, NewInterpreterError>
// {
//     let mut config = OxidizedPythonInterpreterConfig::default();
//     config.interpreter_config.parse_argv = Some(false);
//     config.set_missing_path_configuration = false;

//     pyembed::MainPythonInterpreter::new(config)
// }

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

pub mod conf;
pub mod constant;
pub mod file;
pub mod result;
pub mod view;
pub mod window;

#[cfg(feature = "python")]
pub mod python;

pub use conf::*;
pub use constant::*;
pub use result::*;
pub use view::*;

#[cfg(feature = "python")]
pub use python::*;

/** Start a libnov process.

`res` is a default return result we would like to use
in some particular cases.

`f` is a closure we would like to run as a libnov
process.

`returns` a `Result` specifying if the closure finished
running with a `NovResultSuccess` or a `NovResultError`. */
pub fn main<T: ViewKind>(res: NovResult, f: fn(&mut T, NovResult) -> NovResult) -> NovResult {
    println!("libnov process started.");

    let mut view = view::new();

    f(&mut view, res)
}

/** Handle the result of a finished libnov process.

`returns` an exit code suitable for passing back to
the parent shell or execution environment as a status
indicator of whether the program exited with an error
or not. */
pub fn exit(res: NovResult) -> i32 {
    res.map_or_else(
        |err| {
            let (res, code) = err;
            eprintln!("libnov process exited with error: {}\n{}", &res, code);
            code
        },
        |_res| {
            const CODE: i32 = 0;
            println!("libnov process successful exit.\n{}", CODE);
            CODE
        },
    )
}

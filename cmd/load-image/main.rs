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

extern crate libnov;

use libnov::{conf::*, window::*};

/** An example which opens a window with an image
loaded in it.

You can specify which image you want to load by
passing the image filename as the first command
line argument, otherwise a default image will be
loaded. */
fn main() {
    println!(
        "Starting load-image...

You can run this with a filename as the first \
argument to specify which image to load, otherwise \
a default image will be loaded.
"
    );

    let res = libnov::main(Ok(()), |res| {
        /* This function never returns, so we pass in a default
        exit result to display when the window closes. */
        Window::new(NovConf::default()).open_image(res.clone());

        res
    });

    std::process::exit(libnov::exit(res));
}

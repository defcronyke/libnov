extern crate libnov;

use libnov::window;

/** An example which opens a window with an image
loaded in it.

You can specify which image you want to load by
passing the image filename as the first command
line argument, otherwise a default image will be
loaded. */
fn main() {
    println!("Starting load-image...");
    println!(
        "\nYou can run this with a filename as the first argument \
to specify which image to load, otherwise a default image will be loaded.\n"
    );

    let res = libnov::main(Ok(()), |res| {
        /* This function never returns, so we pass in a default
        exit result to display when the window closes. */
        window::open_image(res.clone());

        res
    });

    std::process::exit(libnov::exit(res));
}

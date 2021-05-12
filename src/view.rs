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

#[cfg(feature = "python")]
pub use crate::python;

// #[cfg(not(feature = "python"))]
// use std::marker::PhantomData;

pub trait ViewKind {
    fn new() -> Self;
    fn get_name(&self) -> String;
}

pub fn new<T: ViewKind>() -> T {
    T::new()
}

// pub struct View<'a, 'b, 'c> {
pub struct View {
    name: String,
    // #[cfg(feature = "python")]
    // pub python: pyembed::MainPythonInterpreter<'a, 'b, 'c>,

    // #[cfg(not(feature = "python"))]
    // _python: (
    //     PhantomData<&'a bool>,
    //     PhantomData<&'b bool>,
    //     PhantomData<&'c bool>,
    // ),
}

// impl<'a, 'b, 'c> ViewKind for View<'a, 'b, 'c> {
impl ViewKind for View {
    fn new() -> Self {
        let name = "View";

        println!("{} created.", name);

        #[cfg(feature = "python")]
        {
            println!("OPTIONAL FEATURE ENABLED: python");

            python::init();
        }

        Self {
            name: name.to_string(),
            // #[cfg(feature = "python")]
            // python: python::create_interpreter::<'a, 'b, 'c>().unwrap(),

            // #[cfg(not(feature = "python"))]
            // _python: (PhantomData, PhantomData, PhantomData),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

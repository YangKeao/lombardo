#![feature(str_as_mut_ptr)]

extern crate failure;
extern crate tempdir;
#[macro_use]
extern crate log;

pub mod client;
pub mod socket;
pub mod wayland;

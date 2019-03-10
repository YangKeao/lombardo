#![feature(str_as_mut_ptr)]

extern crate failure;
#[macro_use]
extern crate log;

pub mod client;
pub mod socket;
pub mod wayland;

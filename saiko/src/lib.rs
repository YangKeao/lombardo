#![feature(str_as_mut_ptr)]
#![feature(stmt_expr_attributes)]

extern crate failure;
#[macro_use]
extern crate log;
extern crate byteorder;
extern crate nix;

pub mod client;
pub mod socket;
mod unix_socket;
pub mod wayland;

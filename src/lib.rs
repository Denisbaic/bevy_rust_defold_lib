#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod bevy_app_config;
pub mod bevy_cpp_interface;
pub mod board;
pub mod defold;
pub mod defold_cpp_interface;
pub mod graph;
pub mod idir2;
pub mod particles;
pub mod world_sides;

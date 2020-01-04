#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

#[macro_use]
extern crate strum_macros;

pub mod track;
pub mod car_type;
pub mod language;
pub mod car_physics;
pub mod data_service;
pub mod gui;
pub mod console;
pub mod template;
pub mod image;

fn main() {
    console::attach();
    let ret = std::panic::catch_unwind(gui::spawn_gui);
    console::free();

    if ret.is_err() {
        std::process::exit(1);
    }
}
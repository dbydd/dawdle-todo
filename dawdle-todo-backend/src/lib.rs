#![feature(fs_try_exists)]
mod configurations;
mod database;
mod modifiers;
mod task;

#[macro_use]
extern crate diesel;

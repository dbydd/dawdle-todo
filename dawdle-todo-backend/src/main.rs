#![feature(fs_try_exists)]

mod configurations;
mod modifiers;
mod task;

fn main() {
    let mut container = task::TaskContainers::new();
    loop {}
    container.write_to_configs();
}

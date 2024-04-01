use std::{
    f32::consts::E,
    fs::{DirEntry, ReadDir},
    vec,
};

use serde_json::map::Entry;

use crate::configurations;

struct TaskInner {
    id: String,
    task_type: TaskType,
    modifiers: [String],
}

enum TaskType {
    Daily,
    Once,
    Heapify,
}

impl TaskInner {
    pub(crate) fn init() {
        configurations::get_configs_at("tasks").flat_map(|dir| match dir {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) if file_type.is_file() => std::fs::read_to_string(entry.path())?,
                Ok(file_type) if file_type.is_dir() => {}
                Err(err) => println!("err!: {err}"),
                _ => {}
            },
            Err(err) => panic!("err at file:{}", err),
        });
    }

    fn read_folder(dir: ReadDir) {}
}

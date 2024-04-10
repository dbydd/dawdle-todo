use std::{
    any::Any, borrow::BorrowMut, cell::OnceCell, collections::HashMap, hash::Hash, time::SystemTime,
};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::configurations;

pub struct TaskContainers {
    inners: HashMap<String, TaskInner>,
    id_path_map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct TaskInner {
    id: String,
    task_type: TaskType,
    init_prio: usize,
    modifiers: Vec<String>,
}

struct Task<'a> {
    inner: &'a TaskInner,
    priority: Option<usize>,
    completed: bool,
    last_completed_time: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize)]
enum TaskType {
    Daily,
    Once,
    Heapify,
}

impl TaskContainers {
    pub(crate) fn new() -> Self {
        let mut container = Self {
            inners: HashMap::new(),
            id_path_map: HashMap::new(),
        };

        container.read_configs();
        container
    }

    pub(crate) fn write_to_configs(&mut self) {
        self.inners.iter_mut().for_each(|e| {
            configurations::save_to(
                self.id_path_map.get(e.0).unwrap(),
                &serde_json::to_string_pretty(&e.1).unwrap(),
            );
        });
    }

    fn read_configs(&mut self) {
        configurations::get_configs_at("tasks", |s| {
            let inner = TaskInner::new(&s.1);
            self.id_path_map
                .insert(inner.id.to_string(), s.0.to_owned());
            self.inners.insert(inner.id.to_string(), inner);
        });
    }
}

impl TaskInner {
    pub(crate) fn new(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

use std::{cell::OnceCell, collections::HashMap, time::SystemTime};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::configurations;

lazy_static! {
    static ref INNERS: HashMap<String, TaskInner> = {
        let mut map = HashMap::new();
        TaskInner::init(|i| {
            map.insert(i.id.to_owned(), i);
        });
        map
    };
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
    priorty: Option<usize>,
    completed: bool,
    last_completed_time: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize)]
enum TaskType {
    Daily,
    Once,
    Heapify,
}

impl TaskInner {
    pub(crate) fn init<I>(mut into_map: I)
    where
        I: FnMut(TaskInner),
    {
        configurations::get_configs_at("tasks", |s| {
            into_map(serde_json::from_str(&s).unwrap());
            ()
        });
    }
}

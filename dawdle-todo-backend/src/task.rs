use std::{
    any::Any, borrow::BorrowMut, cell::OnceCell, collections::HashMap, hash::Hash, time::SystemTime,
};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::configurations;

static mut INNERS: Option<HashMap<String, TaskInner>> = None;
static mut ID_PATH_MAP: Option<HashMap<String, String>> = None;

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

impl TaskInner {
    pub(crate) fn new(id: String, task_type: TaskType, init_prio: usize, modifiers: Vec<String>) {
        let task_inner = TaskInner {
            id,
            task_type,
            init_prio,
            modifiers,
        };
        let serialized = serde_json::to_string_pretty(&task_inner);
    }

    fn inited() -> bool {
        unsafe { INNERS.is_some() && ID_PATH_MAP.is_some() }
    }

    pub(crate) fn write_to_configs() {
        if (Self::inited()) {
            unsafe {
                &mut INNERS.as_ref().inspect(|m| {
                    m.into_iter().for_each(|e| {
                        configurations::save_to(
                            ID_PATH_MAP.as_ref().unwrap().get(e.0).unwrap(),
                            &serde_json::to_string_pretty(e.1).unwrap(),
                        );
                    });
                });
            }
        }
    }

    pub(crate) fn read_configs() {
        let mut id_inner_map: HashMap<String, TaskInner> = HashMap::new();
        let mut id_path_map: HashMap<String, String> = HashMap::new();
        configurations::get_configs_at("tasks", |s| {
            let inner: TaskInner = serde_json::from_str(&s.1).unwrap();
            id_path_map.insert(inner.id.to_string(), s.0.to_owned());
            id_inner_map.insert(inner.id.to_string(), inner);
        });
        unsafe {
            INNERS = Some(id_inner_map);
            ID_PATH_MAP = Some(id_path_map);
        }
    }
}

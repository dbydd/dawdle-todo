use std::vec;

struct TaskInner {
    id: String,
    task_type: TaskType,
}

enum TaskType {
    Daily,
    Once,
    Heapify,
}

impl TaskInner {}

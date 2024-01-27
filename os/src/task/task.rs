use super::TaskContext;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Finished,
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}

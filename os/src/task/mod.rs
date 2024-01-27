mod context;
mod switch;
mod task;

use core::num;

use crate::loader::init_task_cx;
use crate::sync::UPSafeCell;
pub use context::TaskContext;
use lazy_static::lazy_static;
use task::{TaskControlBlock, TaskStatus};
const MAX_TASK_NUM: usize = 16;

pub struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    num_task: usize,
    current_task: usize,
    tcbs: [TaskControlBlock; MAX_TASK_NUM],
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_task = get_num_task_from_kernel();
        let mut tcbs = [TaskControlBlock {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::default(),
        }; MAX_TASK_NUM];

        for i in 0..num_task {
            tcbs[i].task_status = TaskStatus::Ready;
            tcbs[i].task_cx = TaskContext::restore_init(init_task_cx(i));
        }
        TaskManager {
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    num_task,
                    current_task: 0,
                    tcbs,
                })
            },
        }
    };
}

fn get_num_task_from_kernel() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe {
        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = num_app_ptr.read_volatile();
        num_app
    }
}

impl TaskManager {
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tcbs[current].task_status = TaskStatus::Finished;
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tcbs[current].task_status = TaskStatus::Ready;
    }
}

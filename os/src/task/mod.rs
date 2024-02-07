mod context;
mod switch;
mod task;

use crate::sbi::shutdown;
use crate::utils::UPSafeCell;
use crate::{loader::init_task_cx, trap::TrapContext};
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

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tcbs[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::default();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            switch::__switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

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

    fn run_next_task(&self) {
        if let Some(next) = self.get_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.current_task = next;
            inner.tcbs[next].task_status = TaskStatus::Running;
            let current_task_cx_ptr = &mut inner.tcbs[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tcbs[next].task_cx as *const TaskContext;
            drop(inner);
            unsafe {
                switch::__switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All tasks are exited, shutting down...");
            shutdown(false);
        }
    }

    fn get_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let num_task = inner.num_task;
        let current = inner.current_task;
        (current + 1..current + num_task + 1)
            .map(|id| id % num_task)
            .find(|id| inner.tcbs[*id].task_status == TaskStatus::Ready)
    }
}

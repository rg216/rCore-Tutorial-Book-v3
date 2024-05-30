//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::timer::{self, get_time_ms};
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

/// The task manager, where all the tasks are managed.
///
/// Functions implemented on `TaskManager` deals with all task state transitions
/// and task context switching. For convenience, you can find wrappers around it
/// in the module level.
///
/// Most of `TaskManager` are hidden behind the field `inner`, to defer
/// borrowing checks to runtime. You can see examples on how to use `inner` in
/// existing functions on `TaskManager`.
pub struct TaskManager {
    /// total number of tasks
    num_app: usize,
    /// use inner value to get mutable access
    inner: UPSafeCell<TaskManagerInner>,
}

/// Inner of Task Manager
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
}

/// The user app time statistics
pub struct UserStateTime {
    start_time: usize,
    user_time_ms: usize,
    total_usr_time: usize
}

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            runned_time_ms: 0
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
    /// Global variable: USER_RUNNED_TIME
    pub static ref USER_RUNNED_TIME: UPSafeCell<UserStateTime> = unsafe {
        UPSafeCell::new(UserStateTime {
            start_time: 0,
            user_time_ms: 0,
            total_usr_time: 0
        })
    }; 
}

impl TaskManager {
    /// Run the first task in task list.
    ///
    /// Generally, the first task in task list is an idle task (we call it zero process later).
    /// But in ch3, we load apps statically, so the first task is a real app.
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        let mut user_runned_time = USER_RUNNED_TIME.exclusive_access();
        user_runned_time.user_time_ms = timer::get_time_ms();
        user_runned_time.start_time = user_runned_time.user_time_ms;

        drop(user_runned_time);
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        log::info!("start first task : { }", 0);
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    /// Change the status of current `Running` task into `Ready`.
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    /// Find next task to run and return task id.
    ///
    /// In this case, we only return the first `Ready` task in task list.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// Switch current `Running` task to the task we have found,
    /// or there is no `Ready` task and we can exit with all applications completed
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            
            let mut user_runned_time = USER_RUNNED_TIME.exclusive_access();
            let cur_time_ms = timer::get_time_ms();
            let delta_ms = cur_time_ms - user_runned_time.user_time_ms;
            
            user_runned_time.user_time_ms = cur_time_ms;

            inner.tasks[current].runned_time_ms += delta_ms;
            user_runned_time.total_usr_time += delta_ms;

            drop(user_runned_time);
            drop(inner);
            
            if current != next {
                log::info!("switch from task: {} to task: {}",current, next);
            }
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            println!("All applications completed!");
            show_task_time_statistics();
            show_total_user_time();
            let total_time = get_time_ms() - self.get_start_time();
            log::info!("Total kernel time was {} ms", total_time - get_total_user_time());
            shutdown(false);
        }
    }

    fn get_task_running_time(&self, task_id: usize) -> usize {
        let inner = self.inner.exclusive_access();
        inner.tasks[task_id].runned_time_ms
    }

    fn get_user_time(&self) -> usize {
        let user_runned_time = USER_RUNNED_TIME.exclusive_access();
        user_runned_time.total_usr_time
    }

    fn get_start_time(&self) -> usize {
        let user_runned_time = USER_RUNNED_TIME.exclusive_access();
        user_runned_time.start_time
    }
}

/// run first task
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// rust next task
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// exit current task
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// suspend current task, then run next task
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// exit current task,  then run next task
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

/// show time statistics of individual task 
pub fn show_task_time_statistics () {
    for i in 0..get_num_app(){
        let time = TASK_MANAGER.get_task_running_time(i);
        log::info!("Task {} has runned for {} ms", i, time);
    }
}

/// get total user time
pub fn get_total_user_time() -> usize {
    TASK_MANAGER.get_user_time()
}

/// show total user time 
pub fn show_total_user_time() {
    log::info!("Total user time was {}", TASK_MANAGER.get_user_time());
}
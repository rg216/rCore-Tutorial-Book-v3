use super::TaskContext;
use crate::syscall::SyscallInfo;
use crate::config::MAX_SYSCALL_NUM;
use crate::task::TaskInfo;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info: TaskInfo,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskInfo {
    pub fn new(id: usize) -> Self {
        TaskInfo {
            id:id,
            status: TaskStatus::UnInit,
            calls: [SyscallInfo {id: 0, times: 0}; MAX_SYSCALL_NUM],
            time: 0
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_status(&self) -> TaskStatus {
        self.status
    }
    pub fn get_syscall_info(&self, syscall_id: usize) -> SyscallInfo {
        self.calls[syscall_id]
    }
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    pub fn get_time(&self) -> usize {
        self.time
    }
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
    pub fn set_syscall_info(&mut self, syscall_id: usize, times: usize) {
        self.calls[syscall_id].id = syscall_id;
        self.calls[syscall_id].times = times;
    }
    pub fn set_time(&mut self, time: usize) {
        self.time = time;
    }
}
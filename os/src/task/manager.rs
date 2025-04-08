//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use lazy_static::*;

#[allow(unused)]
trait TaskManager {
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>>;
    fn add(&mut self, task: Arc<TaskControlBlock>);
}

pub struct TaskManagerHeap {
    ///A array of `TaskControlBlock` that is thread-safe
    ready_queue: BinaryHeap<Arc<TaskControlBlock>>,
}

impl TaskManagerHeap {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(task);
    }
    
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let t = self.ready_queue.pop();
        // if let Some(ref t) = t {
        //     println!("[task manager]: get task with stride {}",t.get_stride());
        // }
        t
    }
}

impl TaskManager for TaskManagerHeap {
    fn add(&mut self, task: Arc<TaskControlBlock>) {
        TaskManagerHeap::add(self,task);
    }
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        TaskManagerHeap::fetch(self)
    }
}

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManagerDeque {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
#[allow(unused)]
impl TaskManagerDeque {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

impl TaskManager for TaskManagerDeque {
    /// Add process back to ready queue
    fn add(&mut self, task: Arc<TaskControlBlock>) {
        TaskManagerDeque::add(self,task);
    }
    /// Take a process out of the ready queue
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        TaskManagerDeque::fetch(self)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    // pub static ref TASK_MANAGER: UPSafeCell<TaskManagerDeque> =
    //     unsafe { UPSafeCell::new(TaskManagerDeque::new()) };
    pub static ref TASK_MANAGER: UPSafeCell<TaskManagerHeap> =
        unsafe { UPSafeCell::new(TaskManagerHeap::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

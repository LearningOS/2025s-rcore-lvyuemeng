//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.

use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::binary_heap::BinaryHeap;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use lazy_static::*;

#[allow(unused)]
trait TaskManager {
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>>;
    fn add(&mut self, task: Arc<TaskControlBlock>);
}

pub struct TaskManagerHeap {
    ready_queue: BinaryHeap<Arc<TaskControlBlock>>,
}

impl TaskManagerHeap {
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
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        TaskManagerDeque::fetch(self)
    }

    fn add(&mut self, task: Arc<TaskControlBlock>) {
        TaskManagerDeque::add(self, task);
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManagerHeap> =
        unsafe { UPSafeCell::new(TaskManagerHeap::new()) };
    /// PID2PCB instance (map of pid to pcb)
    pub static ref PID2TCB: UPSafeCell<BTreeMap<usize, Arc<TaskControlBlock>>> =
        unsafe { UPSafeCell::new(BTreeMap::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
	//trace!("kernel: TaskManager::add_task");
    PID2TCB
        .exclusive_access()
        .insert(task.getpid(), Arc::clone(&task));
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
	//trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

/// Get process by pid
pub fn pid2task(pid: usize) -> Option<Arc<TaskControlBlock>> {
    let map = PID2TCB.exclusive_access();
    map.get(&pid).map(Arc::clone)
}

/// Remove item(pid, _some_pcb) from PDI2PCB map (called by exit_current_and_run_next)
pub fn remove_from_pid2task(pid: usize) {
    let mut map = PID2TCB.exclusive_access();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2task!", pid);
    }
}

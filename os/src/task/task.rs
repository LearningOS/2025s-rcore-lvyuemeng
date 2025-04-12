//! Types related to task management & Functions for completely changing TCB

use super::id::TaskUserRes;
use super::{kstack_alloc, KernelStack, ProcessControlBlock, TaskContext};
use crate::config::{BIG_STRIDE, PRIORITY};
use crate::trap::TrapContext;
use crate::{mm::PhysPageNum, sync::UPSafeCell};
use alloc::sync::{Arc, Weak};
use core::cell::RefMut;

/// Task control block structure
pub struct TaskControlBlock {
    /// immutable
    pub process: Weak<ProcessControlBlock>,
    /// Kernel stack corresponding to PID
    pub kstack: KernelStack,
    /// mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

#[derive(Clone, Copy)]
pub struct Pass {
    pass: usize,
    priority:usize,
    stride: usize,
}

impl Pass {
    #[allow(unused)]
    fn new(priority: usize) -> Self {
        Self {
            pass: BIG_STRIDE / priority,
            priority,
            stride: 0,
        }
    }
    
    // get stride
    pub fn get_stride(&self) -> usize {
        self.stride
    }

    /// get pass for stride
    pub fn get_pass(&self) -> usize {
        self.pass
    }
    
    /// get priority
    pub fn get_priority(&self) -> usize {
        self.priority
    }

    pub fn set_priority(&mut self, priority: usize) {
        self.pass = BIG_STRIDE / priority;
        self.priority = priority
    }

    pub fn add_stride(&mut self) {
        self.stride = self.stride.wrapping_add(self.pass);
        // println!("[pass]: add pass {}, result {}", self.pass, self.stride);
    }
}

impl Default for Pass {
    fn default() -> Self {
        Self {
            pass: BIG_STRIDE / PRIORITY,
            priority: PRIORITY,
            stride: 0,
        }
    }
}

impl PartialEq for TaskControlBlock {
    fn eq(&self, other: &Self) -> bool {
        let self_stride = self.inner_exclusive_access().pass_data.get_stride();
        let other_stride = other.inner_exclusive_access().pass_data.get_stride();
        self_stride == other_stride
    }
}

impl PartialOrd for TaskControlBlock {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let self_stride = self.inner_exclusive_access().pass_data.get_stride();
        let other_stride = other.inner_exclusive_access().pass_data.get_stride();
        // reverse order
        Some(self_stride.cmp(&other_stride).reverse())
    }
}

impl Eq for TaskControlBlock {}

impl Ord for TaskControlBlock {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let self_stride = self.inner_exclusive_access().pass_data.get_stride();
        let other_stride = other.inner_exclusive_access().pass_data.get_stride();
        // reverse order
        self_stride.cmp(&other_stride).reverse()
    }
}

impl TaskControlBlock {
    /// Get the mutable reference of the inner TCB
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
    /// Get the address of app's page table
    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.memory_set.token()
    }
}

pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    pub pass_data: Pass,
    /// The physical page number of the frame where the trap context is placed
    pub trap_cx_ppn: PhysPageNum,
    /// Save task context
    pub task_cx: TaskContext,

    /// Maintain the execution status of the current process
    pub task_status: TaskStatus,
    /// It is set when active exit or execution error occurs
    pub exit_code: Option<i32>,
}

impl TaskControlBlockInner {
    /// add stride
    pub fn add_stride(&mut self) {
        self.pass_data.add_stride();
    }
    
    pub fn set_priority(&mut self, priority: usize) {
        self.pass_data.set_priority(priority);
    }
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
}

impl TaskControlBlock {
    /// Create a new task
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    pass_data: Pass::default(),
                    res: Some(res),
                    trap_cx_ppn,
                    task_cx: TaskContext::goto_trap_return(kstack_top),
                    task_status: TaskStatus::Ready,
                    exit_code: None,
                })
            },
        }
    }

    /// get priority
    pub fn get_priority(&self) -> usize {
        let inner = self.inner_exclusive_access();
        inner.pass_data.get_priority()
    }

    /// get current stride
    pub fn get_stride(&self) -> usize {
        let inner = self.inner_exclusive_access();
        inner.pass_data.get_stride()
    }
}

#[derive(Copy, Clone, PartialEq)]
/// The execution status of the current process
pub enum TaskStatus {
    /// ready to run
    Ready,
    /// running
    Running,
    /// blocked
    Blocked,
}

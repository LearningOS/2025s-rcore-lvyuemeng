## Lab 5 (Chapter 8)

### Conclusion

Finish Task:
- Inherit previous chapters
- Complete Safety Check Algorithm

Thanks to:
AI: 
- Claude(exposit error message, provide suggestion of structure of `DeadDetect`, provide suggestion of index map from lock to `DeadDetect`); 
- DeepSeek(provide a FP style refactor of safety check algorithm, Give a fully detailed explanation of process and lock problem)

### Q1

Recall `exit_current_and_run_next`:

```rust
    if tid == 0 {
        let pid = process.getpid();
		// ...
        remove_from_pid2process(pid);
        let mut process_inner = process.inner_exclusive_access();
        process_inner.is_zombie = true;
        process_inner.exit_code = exit_code;

		// remove all children to `initproc` process

        let mut recycle_res = Vec::<TaskUserRes>::new();
        for task in process_inner.tasks.iter().filter(|t| t.is_some()) {
            let task = task.as_ref().unwrap();
            // if other tasks are Ready in TaskManager or waiting for a timer to be
            // expired, we should remove them.
            trace!("kernel: exit_current_and_run_next .. remove_inactive_task");
            remove_inactive_task(Arc::clone(&task));
            let mut task_inner = task.inner_exclusive_access();
            if let Some(res) = task_inner.res.take() {
                recycle_res.push(res);
            }
        }

        // need to collect those user res first, then release process_inner
        // for now to avoid deadlock/double borrow problem.
        drop(process_inner);
        recycle_res.clear();

        let mut process_inner = process.inner_exclusive_access();
        process_inner.children.clear();
        process_inner.memory_set.recycle_data_pages();
        process_inner.fd_table.clear();
        process_inner.tasks.clear();
    }
```

Recycled Resource:
- Thread Management:
  - TaskControlBlock
    - User stack
	- Kernel stack
    - Trap Context
- Memory Set
- Process Management:
  - Children
  - File descriptors
- Lock:
  - Mutex
  - Semaphore
  
`TaskControlBlock` of other threads in its reference:

- collected `recycle_res`: remove `task_res` first.
- In process `tasks`: really be cleared by `tasks.clear()`
- In `TaskManager`:  remove by `remove_inactive_task(...)` to reduce reference.
- In Lock `wait_queue`: it's in single process and blocked task are removed when `PCB` deallocated.
- In children process: all moved to `initproc` process to be ready deallocated.

Finally, all resources about task is removed and it will be recycled by rust.

### Q2

`Mutex1`:

it use loop to repeatedly try modifying lock state, but it choose to block current which means current state will never be back in lock state after first time modification. Here, for example: `A` release lock and wake up `B`, but `C` acquire the lock results `B` in wait again, then `A` may repeat wake up `B`...

Notice, `unlock` state is modified before pop queue, so there's the chance that `B` haven't been popped but `C` already modify to `lock` state, results `B` in wait again.


`Mutex2`:

In `Mutex2`, it's the implementation of our real `MutexBlocking`, blocks will be blocked and be released until there's no block in wait queue. problem: `A` release lock and queue is empty, but after `drop(mutex)`, `B` try to lock, now, in `unlock` state but `B` pushed in queue. Or there's another problem that `B` is too fast to get the lock while `A` release fully, then `A` is still in the lock.

Here, if we want strict **Atomic** operation, we need to seek M-level operation to ensure the synchronization of `wakeup` and `lock state`.

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
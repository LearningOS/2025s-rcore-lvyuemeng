## Lab 4

### Conclusion

Finish Task:
- `Inode`: `is_dir`, `is_file`, `add_link`, `remove_link`
- `File`: add `stat` method
- `OSInode`: add `ino`, `nlink`, `is_dir`, `is_file`
- `ROOT_INODE`: `add_link`, `remove_link`
- Syscall: `sys_linkat`, `sys_unlinkat`, `sys_stat`

Thanks to:
- AI: DeepSeek(Thanks to exposit part of error message.)

### Q1

Root inode is denoted as the root dir of our file system. If part of content(`DirEntry`) lost, it lost ability to route to the correct data.

## Lab 5

Thanks to:
- AI: DeepSeek(Thanks to give a finer design of shared message queue for each process)

### Q1

`|` is used to pipe output as input.

For example: `cat file.txt | wc -l`

However, we could directly use `wc <filename> -l` to get line counts.

---

How to design a new structure to build communication for each process?

Introduce a new central queue. Each process will send each message to the queue rather to each other.

**Workflow**:

- process will send its message and target `pid` to the queue.
- queue identify message and write to buffer of each process.
- process will read its buffer.

We need to design atomic operation for each process and maximally reduce possible collision.
So, handling loop will send data to each process while a identification atomic boolean is false.(However I don't know how to implement this concretely, which may be in next chapter :( )
Or **Sharding**, divide to multiple sub queue; **Batching**, do multiple operations in lock or one atomic operation.

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
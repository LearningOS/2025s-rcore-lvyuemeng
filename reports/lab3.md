## Lab3

### Q1:

> [rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0

```
/// 由于 rustsbi 的问题，该程序无法正确退出
/// > rustsbi 0.2.0-alpha.1 已经修复，可以正常退出
```

So in current rustsbi, it will exit normally without any panic behavoir.

Here the reference in output:
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

We can see it's identified as illegal instruction in Trap process.

### Q2:

#### (1):

`sp` is the stack pointer which designed in `riscv`, currently it will point to `TrapContext` in `KernelStack`.

// (refer DeepSeek)
- restore from S-level to U-level.
- restore from switch new TaskContext.

#### (2):

It load the stored CSR info from `TrapContext` to `t0-t2` register and then assign to CSR.

- `sstatus`: store the privilege level of current OS.
- `sepc`: store the addr of next instruction in U-level.
- `sstratch`: a pointer for temporary storage that designed for storing `KernelStack` and `UserStack` currently it point to `UserStack`

#### (3):

Exception for `x0` and `x4`.
- `x0`: hard code as 0.
- `x4`: only for special case.

#### (4):

`sp` will point to `UserStack` and `sscratch` point to `KernelStack`.

#### (5):

- instruction: `sret`
// (refer DeepSeek)
- reason: `sret` will return the addr and modify CSR related state, in which it will change the level and return the `sepc` for U-level instruction.

#### (6):

`sp` will point to `KernelStack` and `sscratch` will point to `UserStack`.

#### (7):

// (refer DeepSeek, Oh! in Lab1!!)
When U-level, app use `ecall` related, it will turn into Trap to S-level.

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
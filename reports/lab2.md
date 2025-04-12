## Lab 2

### Conclusion

Finish Task：
- `sys_mmap/sys_munmap`
- `sys_trace/sys_get_time`
- add `get_mut_data`
- refactor check for `translate_byte_buffer`

Thanks to:
- AI: DeepSeek, Claude(Help to read panic info and provide suggestion for achieving `get_data` by `translate_byte_buffer`), Kimi(Help to provide page related infos of RISC-V)

### Q1:

Page Entry to bundle permission and physical page.

![](/rCore-Blog/assets/Lab4-4.png)

```rust
pub fn ppn(&self) -> PhysPageNum {
	(self.bits >> 10 & ((1usize << 44) - 1)).into()
}
pub fn flags(&self) -> PTEFlags {
	PTEFlags::from_bits(self.bits as u8).unwrap()
}
```

- `V`: valid or not.
- `R/W/X`: control read/write/execution.
- `U`: allowed by U-level to access.
- `G`: global mapping that exist in all address spaces.
- `A`(accessed): whether be accessed after set to zero.
- `D`(Dirty): whether be modified after set to zero.

### Q2:

mcause(Thanks to DeepSeek):

- Instruction Page Fault: cpu try to take instruction from unmapped page.
- Load Page Fault: cpu try to load data from unmapped page.
- Store/AMO Page Fault: cpu try to write data(or AMO) from unmapped page.

`mtval`: store virtual address in error.
`mepc`(spec): point to instruction address that trigger error.(s.t. `ld`,`lw`,`sw` etc...)

---

Benefit of Lazy Load(Thanks to Kimi):
- Reduce memory load.
- Reduce IO operations.
- Speed start.(only load necessary code)
- Reduce page swap(when no enough space, swap memory to disk.)

Lazy Load of `mmap`:

For 10GB allocation:

$4\text{KB}*512 = 2\text{MB} \quad 4\text{KB}*512^2 = 1\text{GB}$
$ 10 \text{GB}(\frac{1}{512} + \frac{1}{512^2}) = 20\text{MB} + 40\text{KB}$

Implementation of Lazy Load:

```rust
pub struct MapArea {
    vpn_range: VPNRange,
	used: BTreeMap<VirtPageNum,FrameTracker>,
    unused: Vec<VirtPageNum>,
    map_type: MapType,
    map_perm: MapPermission,
}
```
when alloced, init as `unused`, then if operation take memory, we will really map it to one `FrameTracker`.

---

Behavoir of Swap(Thanks to DeepSeek):

- the page `V` bits(valid) will be set to 0.
- `RSW` is custom bits to indicate infos, in this case, it can be used to store `swap` info.
  
Workflow:
- trigger `V` as 0.
- `stval` will be routed to virtual address for this page.
- Check `swap info to ensure.
- Read memory and set new `ppn`, set `V` to 1.
- Back to `mepc` and reimplement instructions.

### Q3:

**Side Channel Attack**(Thanks to DeepSeek):
Based on hardware bugs or time, power consumption etc... infos, stealing data bypassing permission isolation.

**Two-Page Table**(Informal Rather: **Kernel page-table isolation**-KPTI):
Isolate kernel and user, so even cpu can't access kernel infos through user page table.

**Single-Page Table**:
Every tasks shared page table with Kernel space. But the map of kernel space is fixed for all tasks. Changing `satp` to load new `ppn` root.

Restriction:
Set `U` bits to restrict `U`-level behavoir.

Benefit:
- No need to change table to reduce operations in Trap.
- Only maintain one table for simplicity.

Switch Case:
- Two-Page:
  - U-level S-level switch, for example: Trap.
- Single-Page:
  - switch `satp` only when process switching.
  - No need to switch U-level, S-level switching.

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。



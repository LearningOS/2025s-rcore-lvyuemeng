## Lab 3

### Conclusion

Finish Task：
- Inherit lab2.
- implement `sys_spawn`
- implement stride dispatch algorithm(by `BinaryHeap`)

Thanks to:
- AI: DeepSeek(Thanks to checkout and exposit part of error message.)
- Asistant: Fontlos, 华华(help to figure out vague ci-test info.)

Suggestion: Hope to provide concrete info on ci-test, especially on `reports/lab*.md` format and which case failed. Especially when local test succeed but doesn't provide info to amid the gap.

### Q1

Cond: `pass = 10`. Store in `stride` in `u8`.
`p1.stride = 255`, `p2.stride = 250`, if `p2` implement in one time slice, `p2` will be added and result a overflow and become smaller, result `260 mod 256 = 4`, which will again repeat in `p1`.

If `priority >= 2`, then for one pass must be with bound `pass <= BIG_STRIDE/2"`, then `STRIDE_MAX - STRIDE_MIN <= BIG_STRIDE/2 - 0 <= BIG_STRIDE/2`

Code design:

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let self_stride = ....
		let other_stride = ....
		if self_stride.wrapping_sub(&other_stride) > BIG_STRIDE/2 {
			// reverse
			return Some(core::cmp::Greater);
		}
		Some(self_stride.cmp(&other_stride).reverse())
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。



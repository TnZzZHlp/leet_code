---
date: 2025-05-23
title: "1822. 数组元素积的符号"
---

已知函数 signFunc(x) 将会根据 x 的正负返回特定值：

如果 x 是正数，返回 1 。
如果 x 是负数，返回 -1 。
如果 x 是等于 0 ，返回 0 。
给你一个整数数组 nums 。令 product 为数组 nums 中所有元素值的乘积。

返回 signFunc(product) 。

示例 1：
输入：nums = [-1,-2,-3,-4,3,2,1]
输出：1
解释：数组中所有值的乘积是 144 ，且 signFunc(144) = 1

示例 2：
输入：nums = [1,5,0,2,-3]
输出：0
解释：数组中所有值的乘积是 0 ，且 signFunc(0) = 0

示例 3：
输入：nums = [-1,1,-1,1,-1]
输出：-1
解释：数组中所有值的乘积是 -1 ，且 signFunc(-1) = -1

提示：
1 <= nums.length <= 1000
-100 <= nums[i] <= 100

### 反思

一开始想着乘积就想到reduce每个元素相乘，但是某个测试用例由于过大导致溢出，看了其他人的题解后发现可以通过统计符号的个数来判断，学到了

还有关于判断数组中是否含某个数，看了下源码，理论上用Vec自带的contains应该比用iter的any方法会快一点，因为源码对于基础类型的对比有优化，不过没有进行测试，差距应该也不大，不过下次遇到需要判断数组是否存在还是用contains方法比较好，因为其他类型最终还是转iter然后再调用any

```rust
pub(super) trait SliceContains: Sized {
    fn slice_contains(&self, x: &[Self]) -> bool;
}

/// 复杂类型对比
impl<T> SliceContains for T
where
    T: PartialEq,
{
    default fn slice_contains(&self, x: &[Self]) -> bool {
        x.iter().any(|y| *y == *self)
    }
}

/// 基本类型对比
impl SliceContains for u8 {
    #[inline]
    fn slice_contains(&self, x: &[Self]) -> bool {
        memchr::memchr(*self, x).is_some()
    }
}

/// 基本类型对比
impl SliceContains for i8 {
    #[inline]
    fn slice_contains(&self, x: &[Self]) -> bool {
        let byte = *self as u8;
        // SAFETY: `i8` and `u8` have the same memory layout, thus casting `x.as_ptr()`
        // as `*const u8` is safe. The `x.as_ptr()` comes from a reference and is thus guaranteed
        // to be valid for reads for the length of the slice `x.len()`, which cannot be larger
        // than `isize::MAX`. The returned slice is never mutated.
        let bytes: &[u8] = unsafe { from_raw_parts(x.as_ptr() as *const u8, x.len()) };
        memchr::memchr(byte, bytes).is_some()
    }
}

macro_rules! impl_slice_contains {
    ($($t:ty),*) => {
        $(
            impl SliceContains for $t {
                #[inline]
                fn slice_contains(&self, arr: &[$t]) -> bool {
                    // Make our LANE_COUNT 4x the normal lane count (aiming for 128 bit vectors).
                    // The compiler will nicely unroll it.
                    const LANE_COUNT: usize = 4 * (128 / (size_of::<$t>() * 8));
                    // SIMD
                    let mut chunks = arr.chunks_exact(LANE_COUNT);
                    for chunk in &mut chunks {
                        if chunk.iter().fold(false, |acc, x| acc | (*x == *self)) {
                            return true;
                        }
                    }
                    // Scalar remainder
                    return chunks.remainder().iter().any(|x| *x == *self);
                }
            }
        )*
    };
}

impl_slice_contains!(u16, u32, u64, i16, i32, i64, f32, f64, usize, isize, char);
```

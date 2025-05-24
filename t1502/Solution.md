给你一个数字数组 arr 。

如果一个数列中，任意相邻两项的差总等于同一个常数，那么这个数列就称为 等差数列 。

如果可以重新排列数组形成等差数列，请返回 true ；否则，返回 false 。

示例 1：
输入：arr = [3,5,1]
输出：true
解释：对数组重新排序得到 [1,3,5] 或者 [5,3,1] ，任意相邻两项的差分别为 2 或 -2 ，可以形成等差数列。

示例 2：
输入：arr = [1,2,4]
输出：false
解释：无法通过重新排序得到等差数列。

提示：
2 <= arr.length <= 1000
-10^6 <= arr[i] <= 10^6

### 反思

思路其实没什么问题，都是获取两项然后相减，不过看其他人的题解发现了 Vec 的 sort_unstable 方法，相对于普通的 sort 方法这个方法要快一点
还有就是 Vec 的 windows 方法，原来可以创建一个 Vec 的子 Vec 学到了，不过感觉这样的话不如直接循环快，毕竟要调用这么多次，不过大部分场景几乎没有区别
而且这样写可读性更高，下次遇到了还是用 windows 这个方法实现吧

[sort_unstable]([https://doc.rust-lang.org/std/primitive.slice.html#method.windows](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable))
[windos Method](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)

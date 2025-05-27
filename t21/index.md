---
date: 2025-05-27
title: 21. 合并两个有序链表
categories: ["LeetCode"]
---

将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

示例 1：
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]

示例 2：
输入：l1 = [], l2 = []
输出：[]

示例 3：
输入：l1 = [], l2 = [0]
输出：[0]

提示：
两个链表的节点数目范围是 [0, 50]
-100 <= Node.val <= 100
l1 和 l2 均按 非递减顺序 排列

### 反思

一开始把整个链表都存进了数组导致每次都要克隆开销很大，后面想到只存链表内的数据再新建一个链表会快不少，但是还是不是最优解

如果原链表不是升序的话又是另一种情况了

下面这种才是最优解

#### 递归

```rust
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None, // 如果两个链表都为空，返回空
            (Some(node1), None) => Some(node1), // 如果第二个链表为空，返回第一个链表
            (None, Some(node2)) => Some(node2), // 如果第一个链表为空，返回第二个链表
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    Some(node2)
                }
            }
        }
    }
}
```

#### 迭代

```rust
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0); // 用哨兵节点简化代码逻辑
        let mut cur = &mut dummy; // cur 指向新链表的末尾
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur.next = list1.take(); // 把 list1 加到新链表中
                cur = cur.next.as_mut()?;
                list1 = cur.next.take();
            } else { // 注：相等的情况加哪个节点都是可以的
                cur.next = list2.take(); // 把 list2 加到新链表中
                cur = cur.next.as_mut()?;
                list2 = cur.next.take();
            };
        }
        cur.next = list1.or(list2); // 拼接剩余链表
        dummy.next
    }
}
```

作者：灵茶山艾府
链接：<https://leetcode.cn/problems/merge-two-sorted-lists/solutions/2373691/liang-chong-fang-fa-die-dai-di-gui-pytho-wf75/>
来源：力扣（LeetCode）

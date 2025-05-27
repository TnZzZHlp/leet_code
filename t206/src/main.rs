// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut h) = head {
            // 先去找子节点的子节点，直到找到链表末尾
            if let Some(n) = Self::reverse_list(h.next.take()) {
                todo!()
            } else {
                return Some(h);
            }
        } else {
            None
        }
    }
}

struct Solution {}

fn main() {}

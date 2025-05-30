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
        // 递归法
        // fn reverse(
        //     current_node: Option<Box<ListNode>>,
        //     previous_node: Option<Box<ListNode>>,
        // ) -> Option<Box<ListNode>> {
        //     match current_node {
        //         Some(mut current_box) => {
        //             // 保存下一个节点，因为 current_box.next 马上会被修改
        //             let next_node = current_box.next.take();
        //             // 当前节点的 next 指向前一个节点
        //             current_box.next = previous_node;
        //             // 递归处理下一个节点，此时的 current_box 变成了 "previous_node"
        //             reverse(next_node, Some(current_box))
        //         }
        //         None => {
        //             // 当 current_node 为 None 时，说明已经到达原链表的末尾
        //             // 此时 previous_node 就是新链表的头节点
        //             previous_node
        //         }
        //     }
        // }
        // // 初始调用时，previous_node 为 None
        // reverse(head, None);

        // 迭代法
        let mut pre = None;
        let mut cur = head;

        while let Some(mut c) = cur {
            let next = c.next.take();
            c.next = pre;
            cur = next;
            pre = Some(c)
        }

        return pre;
    }
}

struct Solution {}

fn main() {}

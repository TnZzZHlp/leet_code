// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 丑陋的实现
        // let mut cur_l1 = l1;
        // let mut cur_l2 = l2;

        // let mut result = Vec::new();
        // let mut carry = 0;

        // loop {
        //     match (cur_l1.take(), cur_l2.take()) {
        //         (Some(n1), Some(n2)) => {
        //             let mut sum = n1.val + n2.val + carry;

        //             if sum > 9 {
        //                 sum -= 10;
        //                 carry = 1;
        //             } else {
        //                 carry = 0;
        //             }

        //             result.push(sum);

        //             cur_l1 = n1.next;
        //             cur_l2 = n2.next;
        //         }
        //         (Some(n1), None) => {
        //             let mut sum = n1.val + carry;

        //             if sum > 9 {
        //                 sum -= 10;
        //                 carry = 1
        //             } else {
        //                 carry = 0
        //             }

        //             result.push(sum);

        //             cur_l1 = n1.next;
        //         }
        //         (None, Some(n2)) => {
        //             let mut sum = n2.val + carry;

        //             if sum > 9 {
        //                 sum -= 10;
        //                 carry = 1
        //             } else {
        //                 carry = 0
        //             }

        //             result.push(sum);
        //             cur_l2 = n2.next
        //         }
        //         (None, None) => {
        //             if carry == 1 {
        //                 result.push(1);
        //             }
        //             break;
        //         }
        //     }
        // }

        // let mut son = None;

        // for v in result.iter().rev() {
        //     let mut cur = Box::new(ListNode::new(*v));

        //     cur.next = son;

        //     son = Some(cur);
        // }

        // son
    }
}

struct Solution {}

fn main() {
    Solution::add_two_numbers(
        creat_list(vec![9, 9, 9, 9, 9, 9, 9]),
        creat_list(vec![9, 9, 9, 9]),
    );
    Solution::add_two_numbers(creat_list(vec![4]), creat_list(vec![5]));
}

fn creat_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut son = None;

    for v in list.iter().rev() {
        let mut cur = Box::new(ListNode::new(*v));

        cur.next = son;

        son = Some(cur);
    }

    son
}

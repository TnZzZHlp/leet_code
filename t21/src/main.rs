// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 先把链表收集到链表，然后排序后再转链表

        let mut result = Vec::new();

        let mut unfold = |list: Option<Box<ListNode>>| {
            if let Some(l) = list {
                let mut cur = l;
                loop {
                    // 把当前项加入到 result
                    result.push(cur.val);

                    if let Some(n) = cur.next {
                        cur = n;
                    } else {
                        break;
                    }
                }
            }
        };

        unfold(list1);
        unfold(list2);

        if result.is_empty() {
            return None;
        }

        result.sort_unstable();
        result.reverse();

        // 创建链表
        let mut last: Option<Box<ListNode>> = None;
        for r in result {
            let mut cur = Box::new(ListNode::new(r));

            if let Some(l) = last {
                cur.next = Some(l)
            }

            last = Some(cur);
        }

        last
    }
}

struct Solution {}

fn main() {
    let mut list1_0 = Box::new(ListNode::new(1));
    let mut list1_1 = Box::new(ListNode::new(2));
    let list1_2 = Box::new(ListNode::new(4));
    list1_1.next = Some(list1_2);
    list1_0.next = Some(list1_1.clone());

    let mut list2_0 = Box::new(ListNode::new(1));
    let mut list2_1 = Box::new(ListNode::new(3));
    let list2_2 = Box::new(ListNode::new(4));
    list2_1.next = Some(list2_2);
    list2_0.next = Some(list2_1.clone());

    print(Solution::merge_two_lists(Some(list1_0), Some(list2_0)));
}

fn print(list: Option<Box<ListNode>>) {
    if let Some(l) = list {
        println!("{}", l.val);
        print(l.next);
    }
}

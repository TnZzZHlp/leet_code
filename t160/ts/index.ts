class ListNode {
    val: number;
    next: ListNode | null;
    constructor(val?: number, next?: ListNode | null) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

function getIntersectionNode(
    headA: ListNode | null,
    headB: ListNode | null
): ListNode | null {
    /// 关键点在于两个指针走过的长度永远是相等的
    if (headA === null || headB === null) return null;
    let pA = headA,
        pB = headB;
    while (pA !== pB) {
        pA = pA === null ? headB : pA.next; //链表A循环结束就循环链表B
        pB = pB === null ? headA : pB.next; //链表B循环结束就循环链表A
    }
    return pA;
    //当pA==pB时就是交点
}

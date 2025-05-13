/*
 * @lc app=leetcode.cn id=2807 lang=rust
 * @lcpr version=30204
 *
 * [2807] 在链表中插入最大公约数
 */

// @lcpr-template-start
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
struct Solution;
fn main() {
    let head = Some(Box::new(ListNode::new(1)));
    assert_eq!(
        Solution::insert_greatest_common_divisors(head),
        Some(Box::new(ListNode::new(1)))
    );
}
// @lcpr-template-end
// @lc code=start
fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        (a, b) = (b, a);
    }
    while b != 0 {
        a %= b;
        (a, b) = (b, a);
    }
    a
}
impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let Some(mut head) = head else {
            return None;
        };
        let Some(tail) = head.next.take() else {
            return Some(head);
        };
        let mut new_node = ListNode::new(gcd(head.val, tail.val));
        new_node.next = Self::insert_greatest_common_divisors(Some(tail));
        head.next = Some(Box::new(new_node));
        Some(head)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [18,6,10,3]\n
// @lcpr case=end

// @lcpr case=start
// [7]\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=LCR 136 lang=rust
 * @lcpr version=30204
 *
 * [LCR 136] 删除链表的节点
 */

// @lcpr-template-start
use leetcode_rs::list;

struct Solution;
fn main() {
    assert_eq!(Solution::delete_node(list![4, 5, 1, 9], 5), list![4, 1, 9]);
}
// @lcpr-template-end
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
// @lc code=start
impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let Some(mut head) = head else {
            return None;
        };
        if head.val == val {
            return head.next;
        }
        head.next = Self::delete_node(head.next, val);
        Some(head)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [4,5,1,9]\n5\n
// @lcpr case=end

// @lcpr case=start
// [4,5,1,9]\n1\n
// @lcpr case=end

 */

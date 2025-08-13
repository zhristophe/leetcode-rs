/*
 * @lc app=leetcode.cn id=1721 lang=rust
 * @lcpr version=30204
 *
 * [1721] 交换链表中的节点
 */

use leetcode_rs::list;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::swap_nodes(list![1, 2, 3, 4, 5], 2),
        list![1, 4, 3, 2, 5]
    );
    assert_eq!(Solution::swap_nodes(list![1], 1), list![1]);
    assert_eq!(Solution::swap_nodes(list![1, 2], 2), list![2, 1]);
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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut n = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            n += 1;
            cur = &node.next;
        }
        let mut head = head;
        let mut a = k;
        let mut b = n - k + 1;
        if a == b {
            return head;
        }
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        let mut ra = None;
        let mut i = 1;
        let mut cur = &mut head;
        loop {
            let node = cur.as_mut().unwrap();
            let (val, next) = (&mut node.val, &mut node.next);
            if i == a {
                ra = Some(val);
            }
            cur = next;
            i += 1;
            if i >= b {
                break;
            }
        }
        let rb = Some(&mut cur.as_mut().unwrap().val);

        std::mem::swap(ra.unwrap(), rb.unwrap());

        head
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4,5]\n2\n
// @lcpr case=end

// @lcpr case=start
// [7,9,6,6,7,8,3,0,9,5]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1]\n1\n
// @lcpr case=end

// @lcpr case=start
// [1,2]\n1\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n2\n
// @lcpr case=end

 */

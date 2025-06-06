/*
 * @lc app=leetcode.cn id=23 lang=rust
 * @lcpr version=30204
 *
 * [23] 合并 K 个升序链表
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
    assert_eq!(Solution::merge_k_lists(vec![]), None);
    assert_eq!(Solution::merge_k_lists(vec![None]), None);
    assert_eq!(Solution::merge_k_lists(vec![None, None]), None);
    assert!(
        Solution::merge_k_lists(vec![None, Some(Box::new(ListNode::new(10))), None]).map(|n| n.val)
            == Some(10)
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // 数据量太小，直接拿出来排序就行了
        // 这里还是正常拿
        if lists.is_empty() {
            return None;
        }
        let mut lists = lists;
        let n = lists.len();
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..n {
            if let Some(node) = &lists[i] {
                heap.push((std::cmp::Reverse(node.val), i));
            }
        }
        let mut head = ListNode::new(0); // 为了方便，建立一个虚拟节点
        let mut last = &mut head as *mut ListNode;
        while let Some((_, i)) = heap.pop() {
            let mut head = std::mem::take(&mut lists[i]).unwrap();
            let tail = std::mem::take(&mut head.next);
            if let Some(tail) = tail {
                heap.push((std::cmp::Reverse(tail.val), i));
                lists[i] = Some(tail);
            }
            unsafe {
                (*last).next = Some(head);
                last = (*last).next.as_mut().unwrap().as_mut() as *mut ListNode;
            }
        }

        head.next
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,4,5],[1,3,4],[2,6]]\n
// @lcpr case=end

// @lcpr case=start
// []\n
// @lcpr case=end

// @lcpr case=start
// [[]]\n
// @lcpr case=end

 */

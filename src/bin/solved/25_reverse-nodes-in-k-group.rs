/*
 * @lc app=leetcode.cn id=25 lang=rust
 * @lcpr version=30204
 *
 * [25] K 个一组翻转链表
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
    for v in [
        (vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        (vec![1, 2, 3, 4, 5], vec![2, 1, 4, 3, 5]),
    ] {
        let mut list = ListNode::new(v.0[0]);
        let mut cur = &mut list as *mut ListNode;
        for i in 1..v.0.len() {
            let mut node = Box::new(ListNode::new(v.0[i]));
            let next = node.as_mut() as *mut _;
            unsafe {
                (*cur).next = Some(node);
            }
            cur = next;
        }
        let mut tmp = Solution::reverse_k_group(Some(Box::new(list)), 2);
        let mut ans = vec![];
        while let Some(node) = tmp {
            ans.push(node.val);
            tmp = node.next;
        }
        assert_eq!(ans, v.1);
    }
}
// @lcpr-template-end
// @lc code=start

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Rust的链表真的是垃圾啊
        // 直接返回
        if k == 1 {
            return head;
        }
        // 建立一个虚拟起点
        let mut vn = ListNode::new(0);
        vn.next = head;
        let mut prev = &mut vn as *mut ListNode;
        loop {
            let mut cur = prev;
            for _ in 0..k {
                if let Some(n) = unsafe { &mut (*cur).next } {
                    cur = n.as_mut() as *mut _;
                } else {
                    return vn.next;
                }
            }
            // 翻转[beg, end]
            let beg = unsafe { (*prev).next.as_mut().unwrap().as_mut() as *mut ListNode };
            let mut prv = unsafe { (*prev).next.take().unwrap() };
            let mut cur = prv.as_mut().next.take();
            for _ in 1..k {
                let nxt = cur.as_mut().unwrap().next.replace(prv);
                prv = cur.unwrap();
                cur = nxt;
            }
            // 连接翻转后的链表
            unsafe {
                (*prev).next = Some(prv);
                (*beg).next = cur;
            }
            prev = beg;
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4,5]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5]\n3\n
// @lcpr case=end

 */

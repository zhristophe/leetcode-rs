/*
 * @lc app=leetcode.cn id=515 lang=rust
 * @lcpr version=30204
 *
 * [515] 在每个树行中找最大值
 */

// @lcpr-template-start
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;
fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::largest_values(root), vec![1]);
    assert_eq!(Solution::largest_values(None), vec![]);
}
// @lcpr-template-end
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        pub fn solve(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, max_v: &mut Vec<i32>) {
            let Some(root) = root else {
                return;
            };
            let val = root.borrow().val;
            if max_v.len() <= depth {
                max_v.push(val);
            } else {
                max_v[depth] = max_v[depth].max(val);
            }
            solve(root.borrow().left.clone(), depth + 1, max_v);
            solve(root.borrow().right.clone(), depth + 1, max_v);
        }
        let mut ans = vec![];
        solve(root, 0, &mut ans);

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,2,5,3,null,9]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

 */

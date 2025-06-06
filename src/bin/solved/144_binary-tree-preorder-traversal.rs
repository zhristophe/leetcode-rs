/*
 * @lc app=leetcode.cn id=144 lang=rust
 * @lcpr version=30204
 *
 * [144] 二叉树的前序遍历
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
    assert_eq!(Solution::preorder_traversal(root), vec![1]);
}
// @lcpr-template-end
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(root) = root {
            let root = root.borrow();
            ans.push(root.val);
            ans.extend(Self::preorder_traversal(root.left.clone()));
            ans.extend(Self::preorder_traversal(root.right.clone()));
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,null,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5,null,8,null,null,6,7,9]\n
// @lcpr case=end

// @lcpr case=start
// []\n
// @lcpr case=end

// @lcpr case=start
// [1]\n
// @lcpr case=end

 */

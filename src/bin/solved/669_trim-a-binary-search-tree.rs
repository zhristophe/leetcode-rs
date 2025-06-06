/*
 * @lc app=leetcode.cn id=669 lang=rust
 * @lcpr version=30204
 *
 * [669] 修剪二叉搜索树
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
    assert_eq!(
        Solution::trim_bst(root.clone(), 1, 2).map(|n| n.borrow().val),
        Some(1)
    );
    assert_eq!(Solution::trim_bst(root, 3, 4), None);
}
// @lcpr-template-end
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root) = root else {
            return None;
        };
        let val = root.borrow().val;
        if val < low {
            Solution::trim_bst(root.borrow().right.clone(), low, high)
        } else if val > high {
            Solution::trim_bst(root.borrow().left.clone(), low, high)
        } else {
            let left = root.borrow().left.clone();
            root.borrow_mut().left = Solution::trim_bst(left, low, high);
            let right = root.borrow().right.clone();
            root.borrow_mut().right = Solution::trim_bst(right, low, high);
            Some(root)
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,0,2]\n1\n2\n
// @lcpr case=end

// @lcpr case=start
// [3,0,4,null,2,null,null,1]\n1\n3\n
// @lcpr case=end

 */

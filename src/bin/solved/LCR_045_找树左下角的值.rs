/*
 * @lc app=leetcode.cn id=LCR 045 lang=rust
 * @lcpr version=30204
 *
 * [LCR 045] 找树左下角的值
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
    let root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(Solution::find_bottom_left_value(root), 2);
}
// @lcpr-template-end
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find_leftest(
            root: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            max_depth: &mut i32,
            ans: &mut i32,
        ) {
            let Some(root) = root else {
                return;
            };
            if depth > *max_depth {
                *max_depth = depth;
                *ans = root.borrow().val;
            }
            find_leftest(root.borrow_mut().left.take(), depth + 1, max_depth, ans);
            find_leftest(root.borrow_mut().right.take(), depth + 1, max_depth, ans);
        }
        let mut max_depth = -1;
        let mut ans = 0;
        find_leftest(root, 0, &mut max_depth, &mut ans);
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,1,3]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,null,5,6,null,null,7]\n
// @lcpr case=end

 */

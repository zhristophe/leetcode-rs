/*
 * @lc app=leetcode.cn id=662 lang=rust
 * @lcpr version=30204
 *
 * [662] 二叉树最大宽度
 */

// @lcpr-template-start
use leetcode_rs::tree;

struct Solution;
fn main() {
    assert_eq!(
        Solution::width_of_binary_tree(tree![TreeNode, 1, 3, 2, 5, 3, null, 9]),
        4
    );
    assert_eq!(
        Solution::width_of_binary_tree(tree![TreeNode, 1, 3, 2, 5, null, null, 9, 6, null, 7]),
        7
    );
    assert_eq!(
        Solution::width_of_binary_tree(tree![TreeNode, 1, 3, 2, 5]),
        2
    );
    assert_eq!(
        Solution::width_of_binary_tree(tree![TreeNode, 1, 1, null, 1, 1, null, null]),
        2
    );
}
// @lcpr-template-end
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
// @lc code=start
// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 逐层遍历
        let mut ans = 1;
        let mut layer = vec![(0, root.unwrap())];
        while !layer.is_empty() {
            ans = ans.max(layer.last().unwrap().0 - layer.first().unwrap().0 + 1);
            let mut next_layer = vec![];
            for (idx, node) in layer {
                for (i, n) in [
                    (0, node.borrow().left.clone()),
                    (1, node.borrow().right.clone()),
                ] {
                    if let Some(n) = n {
                        next_layer.push((idx * 2 + i, n));
                    }
                }
            }
            layer = next_layer;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,2,5,3,null,9]\n
// @lcpr case=end

// @lcpr case=start
// [1,3,2,5,null,null,9,6,null,7]\n
// @lcpr case=end

// @lcpr case=start
// [1,3,2,5]\n
// @lcpr case=end

 */

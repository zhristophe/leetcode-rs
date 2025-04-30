/*
 * @lc app=leetcode id=572 lang=rust
 *
 * [572] Subtree of Another Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn same_tree(t1: Tree, t2: Tree) -> bool {
        if t1.is_none() != t2.is_none() {
            return false;
        }
        if t1.is_none() && t2.is_none() {
            return true;
        }
        let t1 = t1.unwrap();
        let t2 = t2.unwrap();
        if t1.borrow().val != t2.borrow().val {
            return false;
        }
        let left1 = t1.borrow().left.clone();
        let left2 = t2.borrow().left.clone();
        if !Self::same_tree(left1, left2) {
            return false;
        }
        let right1 = t1.borrow().right.clone();
        let right2 = t2.borrow().right.clone();
        if !Self::same_tree(right1, right2) {
            return false;
        }
        true
    }
    #[allow(dead_code)]
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let Some(root) = root else {
            return false;
        };
        if Some(root.borrow().val) == sub_root.as_ref().map(|t| t.borrow().val) {
            if Self::same_tree(Some(root.clone()), sub_root.clone()) {
                return true;
            }
        }
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        Self::is_subtree(left, sub_root.clone()) || Self::is_subtree(right, sub_root)
    }
}
// @lc code=end

#[allow(dead_code)]
struct Solution;
fn main() {}

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

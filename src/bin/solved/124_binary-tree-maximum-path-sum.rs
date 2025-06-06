/*
 * @lc app=leetcode.cn id=124 lang=rust
 * @lcpr version=30204
 *
 * [124] 二叉树中的最大路径和
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
    let l = TreeNode::new(2);
    let r = TreeNode::new(3);
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(l)));
    root.right = Some(Rc::new(RefCell::new(r)));
    assert_eq!(Solution::max_path_sum(Some(Rc::new(RefCell::new(root)))), 6);
}
// @lcpr-template-end
// @lc code=start
use std::cell::RefCell;
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 搜索从每个节点出发向下的所有路径
        struct Data {
            ans: i32,
        }
        // 返回值是该节点出发向下的所有路径中的最大和
        fn search(root: Option<Rc<RefCell<TreeNode>>>, data: &mut Data) -> i32 {
            let Some(root) = root else {
                return 0;
            };
            let val = root.borrow().val;
            let left = search(root.borrow().left.clone(), data);
            let right = search(root.borrow().right.clone(), data);
            let res = val.max(val + left).max(val + right);
            let mut ans = val;
            if left > 0 {
                ans += left;
            }
            if right > 0 {
                ans += right;
            }
            data.ans = data.ans.max(ans);
            res
        }
        let mut data = Data { ans: i32::MIN };
        search(root, &mut data);

        data.ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [-10,9,20,null,null,15,7]\n
// @lcpr case=end

 */

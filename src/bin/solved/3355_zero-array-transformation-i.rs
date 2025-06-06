/*
 * @lc app=leetcode.cn id=3355 lang=rust
 * @lcpr version=30204
 *
 * [3355] 零数组变换 I
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::is_zero_array(vec![1, 0, 1], vec2d![[0, 2]]), true);
}
// @lcpr-template-end
// @lc code=start
use std::{collections::HashMap, ops::AddAssign};
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut beg = HashMap::new();
        let mut end = HashMap::new();
        for query in queries {
            beg.entry(query[0]).or_insert(0).add_assign(1);
            end.entry(query[1]).or_insert(0).add_assign(1);
        }
        let mut cnt = *beg.get(&0).unwrap_or(&0);
        if nums[0] > cnt {
            return false;
        }
        for (i, num) in nums.into_iter().enumerate().skip(1) {
            cnt += beg.get(&(i as i32)).unwrap_or(&0);
            cnt -= end.get(&(i as i32 - 1)).unwrap_or(&0);
            if num > cnt {
                return false;
            }
        }

        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,0,1]\n[[0,2]]\n
// @lcpr case=end

// @lcpr case=start
// [4,3,2,1]\n[[1,3],[0,2]]\n
// @lcpr case=end

 */

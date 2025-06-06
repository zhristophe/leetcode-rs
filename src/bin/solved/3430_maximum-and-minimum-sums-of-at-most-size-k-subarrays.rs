/*
 * @lc app=leetcode.cn id=3430 lang=rust
 * @lcpr version=30204
 *
 * [3430] 最多 K 个元素的子数组的最值之和
 */

use std::collections::{BTreeMap, BTreeSet};

// @lcpr-template-start
struct Solution;
fn main() {}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        // 记录每个数在邻域中的最小宽度或者最大宽度
        // 比如 i 在 a..i..b中最大, c..i..d中最小
        let mut min = BTreeSet::new(); // 记录最小数和其位置
        let mut max = BTreeSet::new(); // 同上
        let n = nums.len();
        let mut ranges = vec![(0, 0); n]; // 每个数的最大最小宽度
        let mut ans = 0;
        let mut min = vec![i32::MAX; n];

        fn solve() {}

        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,-3,1]\n2\n
// @lcpr case=end

 */

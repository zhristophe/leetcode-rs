/*
 * @lc app=leetcode.cn id=2485 lang=rust
 * @lcpr version=30204
 *
 * [2485] 找出中枢整数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::pivot_integer(8), 6);
    assert_eq!(Solution::pivot_integer(1), 1);
    assert_eq!(Solution::pivot_integer(4), -1);
}
// @lcpr-template-end
// @lc code=start
use std::ops::Div;
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        // 不难推导，2x^2 = n^2 + n
        let x = ((n * n + n) as f64).div(2.).sqrt() as i32;
        if 2 * x * x == n * n + n {
            x
        } else {
            -1
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// 8\n
// @lcpr case=end

// @lcpr case=start
// 1\n
// @lcpr case=end

// @lcpr case=start
// 4\n
// @lcpr case=end

 */

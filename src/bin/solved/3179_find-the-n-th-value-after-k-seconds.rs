/*
 * @lc app=leetcode.cn id=3179 lang=rust
 * @lcpr version=30204
 *
 * [3179] K 秒后第 N 个元素的值
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
    assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![1; n];
        for _ in 0..k {
            for i in 1..n {
                nums[i] += nums[i - 1];
                nums[i] %= 10_0000_0007;
            }
        }

        nums[n - 1]
    }
}
// @lc code=end

/*
// @lcpr case=start
// 4\n5\n
// @lcpr case=end

// @lcpr case=start
// 5\n3\n
// @lcpr case=end

 */

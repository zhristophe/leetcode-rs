/*
 * @lc app=leetcode.cn id=1262 lang=rust
 * @lcpr version=30204
 *
 * [1262] 可被三整除的最大和
 */

// @lcpr-template-start
struct Solution;
fn main() {
    // assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
    assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = [0; 3];
        for num in nums {
            let m = num as usize % 3;
            let mut next = dp.clone();
            for i in 0..3 {
                let prev = (3 + i - m) % 3;
                let v = if dp[prev] != 0 || i == m {
                    dp[prev] + num
                } else {
                    0
                };
                next[i] = next[i].max(v);
            }
            // dbg!(&next);
            dp = next;
        }
        dp[0]
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,6,5,1,8]\n
// @lcpr case=end

// @lcpr case=start
// [4]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,4]\n
// @lcpr case=end

 */

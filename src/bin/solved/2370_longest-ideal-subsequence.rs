/*
 * @lc app=leetcode.cn id=2370 lang=rust
 * @lcpr version=30204
 *
 * [2370] 最长理想子序列
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
    assert_eq!(Solution::longest_ideal_string("pvjcci".to_string(), 4), 2);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; 26];
        for &c in s.as_bytes() {
            let c = (c - b'a') as usize;
            let start = if c >= k { c - k } else { 0 };
            let end = (26 - 1).min(c + k);
            // dbg!(c, start, end);
            dp[c] = dp[start..=end].iter().max().unwrap_or(&0) + 1;
        }
        dp.iter().max().unwrap().to_owned()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "acfgbd"\n2\n
// @lcpr case=end

// @lcpr case=start
// "abcd"\n3\n
// @lcpr case=end

 */

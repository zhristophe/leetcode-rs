/*
 * @lc app=leetcode.cn id=1930 lang=rust
 * @lcpr version=30204
 *
 * [1930] 长度为 3 的不同回文子序列
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::count_palindromic_subsequence("aabca".to_string()),
        3
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::BTreeSet;
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        // 首尾相同，中间所有字母的种类
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = 0;
        let mut bt = vec![BTreeSet::new(); 26];
        for i in 0..n {
            bt[(s[i] - b'a') as usize].insert(i);
        }
        for i in 0..26 {
            if bt[i].len() < 2 {
                continue;
            }
            let beg = bt[i].first().unwrap();
            let end = bt[i].last().unwrap();
            for j in 0..26 {
                if bt[j].range(beg + 1..*end).next().is_some() {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// "aabca"\n
// @lcpr case=end

// @lcpr case=start
// "adc"\n
// @lcpr case=end

// @lcpr case=start
// "bbcbaba"\n
// @lcpr case=end

 */

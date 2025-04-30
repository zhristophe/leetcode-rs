/*
 * @lc app=leetcode id=2207 lang=rust
 *
 * [2207] Maximize Number of Subsequences in a String
 */

// @lc code=start
impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let (c0, c1) = (
            pattern.chars().nth(0).unwrap(),
            pattern.chars().nth(1).unwrap(),
        );

        let (mut n0, mut n1) = (0, 0);
        let mut ans = 0i64;
        for c in text.chars() {
            if c == c1 {
                n1 += 1;
                ans += n0;
                // dbg!(ans, n0);
            }
            if c == c0 {
                n0 += 1;
            }
        }
        // dbg!(n0, n1);
        ans + n0.max(n1)
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::maximum_subsequence_count("abdcdbc".to_owned(), "ac".to_owned()),
        4
    );
    assert_eq!(
        Solution::maximum_subsequence_count("aa".to_owned(), "aa".to_owned()),
        3
    );
    assert_eq!(
        Solution::maximum_subsequence_count("aaa".to_owned(), "aa".to_owned()),
        6
    );
    assert_eq!(
        Solution::maximum_subsequence_count("aaac".to_owned(), "ac".to_owned()),
        6
    );
}

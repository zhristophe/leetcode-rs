/*
 * @lc app=leetcode.cn id=9 lang=rust
 * @lcpr version=30204
 *
 * [9] 回文数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(10), false);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = (x as u32).to_string();
        let s = s.as_bytes();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// 121\n
// @lcpr case=end

// @lcpr case=start
// -121\n
// @lcpr case=end

// @lcpr case=start
// 10\n
// @lcpr case=end

 */

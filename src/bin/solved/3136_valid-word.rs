/*
 * @lc app=leetcode.cn id=3136 lang=rust
 * @lcpr version=30204
 *
 * [3136] 有效单词
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::is_valid("Ya$".to_string()), false)
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_valid(word: String) -> bool {
        let word = word.to_ascii_lowercase();
        let word = word.as_bytes();
        let n = word.len();
        if n < 3 {
            return false;
        }

        let mut a_cnt = 0;
        let mut b_cnt = 0;
        for i in 0..n {
            match word[i] {
                b'a' | b'e' | b'i' | b'o' | b'u' => a_cnt += 1,
                b'0'..=b'9' => (),
                c => match c {
                    b'a'..=b'z' | b'A'..=b'Z' => b_cnt += 1,
                    _ => return false,
                },
            }
        }

        a_cnt > 0 && b_cnt > 0
    }
}
// @lc code=end

/*
// @lcpr case=start
// "234Adas"\n
// @lcpr case=end

// @lcpr case=start
// "b3"\n
// @lcpr case=end

// @lcpr case=start
// "a3$e"\n
// @lcpr case=end

 */

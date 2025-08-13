/*
 * @lc app=leetcode.cn id=389 lang=rust
 * @lcpr version=30204
 *
 * [389] 找不同
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut cnt = [0; 26];
        for &c in t {
            cnt[(c - b'a') as usize] += 1;
        }
        for &c in s {
            cnt[(c - b'a') as usize] -= 1;
        }
        for i in 0..26 {
            if cnt[i] != 0 {
                return (i as u8 + b'a') as char;
            }
        }

        unreachable!()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abcd"\n"abcde"\n
// @lcpr case=end

// @lcpr case=start
// ""\n"y"\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=1540 lang=rust
 * @lcpr version=30204
 *
 * [1540] K 次操作转变字符串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::can_convert_string("input".to_string(), "ouput".to_string(), 9),
        true
    );
    assert_eq!(
        Solution::can_convert_string("aa".to_string(), "bb".to_string(), 26),
        false
    );
    assert_eq!(
        Solution::can_convert_string("aa".to_string(), "bb".to_string(), 27),
        true
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        // 同余系
        let s = s.as_bytes();
        let t = t.as_bytes();
        if s.len() != t.len() {
            return false;
        }
        let mut cnt = [0; 26];
        for (&a, &b) in s.iter().zip(t.iter()) {
            let d = (b + 26 - a) % 26;
            cnt[d as usize] += 1;
        }
        for i in 1..26 {
            if cnt[i] > (k + 26 - i as i32) / 26 {
                return false;
            }
        }

        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// "input"\n"ouput"\n9\n
// @lcpr case=end

// @lcpr case=start
// "abc"\n"bcd"\n10\n
// @lcpr case=end

// @lcpr case=start
// "aab"\n"bbb"\n27\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=2825 lang=rust
 * @lcpr version=30204
 *
 * [2825] 循环增长使字符串子序列等于另一个字符串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::can_make_subsequence(String::from("abc"), String::from("ad")),
        true
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        // 对于str2[..i]，如果找到最短的父序列str1[..j]，一定是最优情况
        // 因而可以贪心
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let mut i1 = 0;
        for i2 in 0..s2.len() {
            while i1 < s1.len() {
                if s1[i1] == s2[i2] || (s1[i1] == b'z' && s2[i2] == b'a') || (s1[i1] + 1 == s2[i2])
                {
                    break;
                }
                i1 += 1;
            }
            if i1 == s1.len() {
                return false;
            }
            i1 += 1;
        }

        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abc"\n"ad"\n
// @lcpr case=end

// @lcpr case=start
// "zc"\n"ad"\n
// @lcpr case=end

// @lcpr case=start
// "ab"\n"d"\n
// @lcpr case=end

 */

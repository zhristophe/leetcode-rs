/*
 * @lc app=leetcode.cn id=1750 lang=rust
 * @lcpr version=30204
 *
 * [1750] 删除字符串两端相同字符后的最短长度
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::minimum_length("a".to_string()), 1);
    assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    assert_eq!(Solution::minimum_length("cac".to_string()), 1);
    assert_eq!(Solution::minimum_length("aa".to_string()), 0);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut i = 0;
        let mut j = n - 1;
        while i < j {
            if s[i] != s[j] {
                break;
            }
            while i < j && s[i] == s[i + 1] {
                i += 1;
            }
            while i < j && s[j] == s[j - 1] {
                j -= 1;
            }
            i += 1;
            j -= 1;
        }
        if j < i {
            0
        } else {
            (j - i + 1) as i32
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "ca"\n
// @lcpr case=end

// @lcpr case=start
// "cabaabac"\n
// @lcpr case=end

// @lcpr case=start
// "aabccabba"\n
// @lcpr case=end

 */

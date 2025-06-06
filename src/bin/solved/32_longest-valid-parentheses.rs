/*
 * @lc app=leetcode.cn id=32 lang=rust
 * @lcpr version=30204
 *
 * [32] 最长有效括号
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // 复杂度O(n^2)，更好的算法待实现
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = 0;
        let mut cnt = vec![0; n]; // 缓存起点
        cnt.insert(0, 0);
        for i in 0..n {
            for j in 0..=i {
                if cnt[j] < 0 {
                    continue;
                }
                if s[i] == b'(' {
                    cnt[j] += 1;
                } else {
                    cnt[j] -= 1;
                    if cnt[j] == 0 {
                        ans = ans.max(i - j + 1);
                    }
                }
            }
        }

        ans as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// "(()"\n
// @lcpr case=end

// @lcpr case=start
// ")()())"\n
// @lcpr case=end

// @lcpr case=start
// ""\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=20 lang=rust
 * @lcpr version=30204
 *
 * [20] 有效的括号
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("(".to_string()), false);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for &b in s.as_bytes() {
            match b {
                b'(' | b'[' | b'{' => stack.push(b),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "()"\n
// @lcpr case=end

// @lcpr case=start
// "()[]{}"\n
// @lcpr case=end

// @lcpr case=start
// "(]"\n
// @lcpr case=end

// @lcpr case=start
// "([])"\n
// @lcpr case=end

 */

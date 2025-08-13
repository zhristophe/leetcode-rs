/*
 * @lc app=leetcode.cn id=306 lang=rust
 * @lcpr version=30204
 *
 * [306] 累加数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::is_additive_number("112358".to_string()), true);
    assert_eq!(Solution::is_additive_number("199100199".to_string()), true);
    assert_eq!(Solution::is_additive_number("1".repeat(35)), false)
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        // 搜！
        fn solve(a: &[u8], b: &[u8], s: &[u8]) -> bool {
            if s.is_empty() {
                return true;
            }
            let mut c = vec![0; a.len().max(b.len()) + 1];
            for i in 0..c.len() {
                let [ai, bi] = [a, b].map(|a| {
                    if i < a.len() {
                        a.len() - i - 1
                    } else {
                        a.len()
                    }
                });
                let ci = c.len() - i - 1;
                c[ci] += a.get(ai).unwrap_or(&0);
                c[ci] += b.get(bi).unwrap_or(&0);
                if ci > 0 {
                    c[ci - 1] += c[ci] / 10;
                    c[ci] %= 10;
                }
            }
            // dbg!((a, b, &c));
            let c = if c[0] == 0 && c.len() > 1 {
                &c[1..]
            } else {
                &c
            };
            if c.len() > s.len() {
                return false;
            }
            for i in 0..c.len() {
                if c[i] != s[i] {
                    return false;
                }
            }
            let c = &s[..c.len()];
            let s = &s[c.len()..];

            solve(b, c, s)
        }
        let mut num = num;
        let num = unsafe { num.as_bytes_mut() };
        for i in 0..num.len() {
            num[i] -= b'0';
        }
        for i in 1..num.len() {
            if i > 1 && num[0] == 0 {
                break;
            }
            for j in i + 1..num.len() {
                if j - i > 1 && num[i] == 0 {
                    break;
                }
                if solve(&num[..i], &num[i..j], &num[j..]) {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end

/*
// @lcpr case=start
// "112358"\n
// @lcpr case=end

// @lcpr case=start
// "199100199"\n
// @lcpr case=end

 */

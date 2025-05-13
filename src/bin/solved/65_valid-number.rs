/*
 * @lc app=leetcode.cn id=65 lang=rust
 * @lcpr version=30204
 *
 * [65] 有效数字
 */

// @lcpr-template-start
struct Solution;
fn main() {
    for s in vec![
        "2",
        "0089",
        "-0.1",
        "+3.14",
        "4.",
        "-.9",
        "2e10",
        "-90E3",
        "3e+7",
        "+6e-1",
        "53.5e93",
        "-123.456e789",
    ] {
        assert_eq!(Solution::is_number(s.to_string()), true);
    }
    for s in vec!["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"] {
        assert_eq!(Solution::is_number(s.to_string()), false);
    }
}
// @lcpr-template-end
// @lc code=start
use std::iter::Peekable;
impl Solution {
    pub fn is_number(s: String) -> bool {
        fn parse_integer<I: Iterator<Item = char>>(mut s: Peekable<I>) -> Option<Peekable<I>> {
            if matches!(s.peek(), Some('+' | '-')) {
                s.next();
            }
            let mut f = false;
            while matches!(s.peek(), Some('0'..='9')) {
                f = true;
                s.next();
            }
            if f {
                Some(s)
            } else {
                None
            }
        }
        fn parse_10<I: Iterator<Item = char>>(mut s: Peekable<I>) -> Option<Peekable<I>> {
            if matches!(s.peek(), Some('+' | '-')) {
                s.next();
            }
            let mut f = false;
            while matches!(s.peek(), Some('0'..='9')) {
                f = true;
                s.next();
            }
            if s.peek() != Some(&'.') {
                return None;
            } else {
                s.next();
            }
            while matches!(s.peek(), Some('0'..='9')) {
                f = true;
                s.next();
            }
            if !f {
                return None;
            }
            Some(s)
        }
        fn parse_exp<I: Iterator<Item = char>>(mut s: Peekable<I>) -> bool {
            match s.next() {
                None => return true,
                Some('e' | 'E') => {}
                _ => return false,
            }
            parse_integer(s).map(|mut s| s.peek().is_none()) == Some(true)
        }

        let s = s.chars().into_iter().peekable();
        let case1 = {
            let s = parse_integer(s.clone());
            s.map(|s| parse_exp(s)) == Some(true)
        };
        let case2 = {
            let s = parse_10(s);
            s.map(|s| parse_exp(s)) == Some(true)
        };

        case1 || case2
    }
}
// @lc code=end

/*
// @lcpr case=start
// "0"\n
// @lcpr case=end

// @lcpr case=start
// "e"\n
// @lcpr case=end

// @lcpr case=start
// "."\n
// @lcpr case=end

 */

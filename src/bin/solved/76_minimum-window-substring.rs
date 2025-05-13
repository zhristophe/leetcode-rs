/*
 * @lc app=leetcode.cn id=76 lang=rust
 * @lcpr version=30204
 *
 * [76] 最小覆盖子串
 */

use std::usize;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let get_char_byte = |c: u8| -> u8 {
            match c {
                b'a'..=b'z' => c - b'a',
                b'A'..=b'Z' => c - b'A' + 26,
                _ => unreachable!("{c}"),
            }
        };
        let s = s
            .as_bytes()
            .iter()
            .map(|c| get_char_byte(*c))
            .collect::<Vec<_>>();
        let mut t_cnt = [0; 52];
        for c in t.as_bytes() {
            t_cnt[get_char_byte(*c) as usize] += 1;
        }
        let mut s_cnt = [0; 52];
        let mut i = 0;
        let mut j = 0;
        let mut ans_len = usize::MAX;
        let mut ans = (0, 0);
        loop {
            if j >= s.len() {
                break;
            }
            // dbg!(i, j);
            let pos = s[j] as usize;
            s_cnt[pos] += 1;
            // dbg!(pos);
            if s_cnt[pos] >= t_cnt[pos] && s_cnt.iter().zip(t_cnt.iter()).all(|(s, t)| s >= t) {
                while i < j {
                    // dbg!((i, j, s_cnt, t_cnt));
                    let pos = s[i] as usize;
                    // dbg!((i, j, s_cnt[i], t_cnt[i]));
                    if s_cnt[pos] == t_cnt[pos] {
                        break;
                    }
                    s_cnt[pos] -= 1;
                    i += 1;
                }
                let len = j - i + 1;
                if len < ans_len {
                    ans_len = len;
                    ans = (i, j);
                }
            }
            j += 1;
        }

        // dbg!(ans_len, ans);
        if ans_len == usize::MAX {
            "".to_string()
        } else {
            let (i, j) = ans;
            s[i..=j]
                .iter()
                .map(|b| match *b {
                    0..26 => (b'a' + b) as char,
                    26..52 => (b'A' + b - 26) as char,
                    _ => unreachable!(),
                })
                .collect()
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "ADOBECODEBANC"\n"ABC"\n
// @lcpr case=end

// @lcpr case=start
// "a"\n"a"\n
// @lcpr case=end

// @lcpr case=start
// "a"\n"aa"\n
// @lcpr case=end

 */

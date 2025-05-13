/*
 * @lc app=leetcode.cn id=3305 lang=rust
 * @lcpr version=30204
 *
 * [3305] 元音辅音字符串计数 I
 */

// @lcpr-template-start
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let chars = word.chars().collect::<Vec<_>>();
        let f = |k| {
            // 统计>=k的辅音数
            let mut i = 0;
            let mut cnt1 = HashMap::new();
            for ch in "aeiou".chars() {
                cnt1.insert(ch, 0);
            }
            let mut cnt2 = 0;
            let mut ans = 0;
            for j in 0..chars.len() {
                match chars[j] {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        *cnt1.get_mut(&chars[j]).unwrap() += 1;
                    }
                    _ => cnt2 += 1,
                }
                while cnt1.values().all(|v| *v > 0) && cnt2 >= k {
                    match chars[i] {
                        'a' | 'e' | 'i' | 'o' | 'u' => {
                            *cnt1.get_mut(&chars[i]).unwrap() -= 1;
                        }
                        _ => cnt2 -= 1,
                    }
                    i += 1;
                }
                ans += i;
            }
            ans as i32
        };

        f(k) - f(k + 1)
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::count_of_substrings("aeioqq".to_owned(), 1), 0);
    assert_eq!(Solution::count_of_substrings("aeiou".to_owned(), 0), 1);
    assert_eq!(
        Solution::count_of_substrings("ieaouqqieaouqq".to_owned(), 1),
        3
    );
}

/*
// @lcpr case=start
// "aeioqq"\n1\n
// @lcpr case=end

// @lcpr case=start
// "aeiou"\n0\n
// @lcpr case=end

// @lcpr case=start
// "ieaouqqieaouqq"\n1\n
// @lcpr case=end

 */

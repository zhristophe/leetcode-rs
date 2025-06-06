/*
 * @lc app=leetcode.cn id=30 lang=rust
 * @lcpr version=30204
 *
 * [30] 串联所有单词的子串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo", "bar"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec![0, 9]
    );
    assert_eq!(
        Solution::find_substring(
            "a".to_string(),
            vec!["a", "a"].into_iter().map(|s| s.to_string()).collect()
        ),
        vec![]
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // 注意words长度相同，没必要做什么AC自动机
        let (n, m, k) = (s.len(), words[0].len(), words.len());
        let words = {
            let mut tmp = HashMap::new();
            for word in words {
                *tmp.entry(word).or_insert(0) += 1;
            }
            tmp
        };
        // dbg!(&words);

        let mut ans = vec![];
        for beg in 0..m {
            if beg + m * k > n {
                break;
            }
            let mut diff = words.len(); // 数量和words不同的项数
            let mut hash = HashMap::new();
            let n = (n - beg) / m;
            for i in 0..k {
                let word = &s[beg + i * m..beg + (i + 1) * m];
                let cnt = hash.entry(word).or_insert(0);
                if let Some(&v) = words.get(word) {
                    if v == *cnt {
                        diff += 1;
                    }
                    *cnt += 1;
                    if v == *cnt {
                        diff -= 1;
                    }
                }
                // dbg!(&hash, diff);
            }
            if diff == 0 {
                ans.push(beg);
            }
            for j in k..n {
                let i = beg + (j - k) * m;
                let j = beg + j * m;
                let wi = &s[i..i + m];
                // // dbg!(wi);
                // // dbg!(diff);
                let cnt = hash.get_mut(wi).unwrap();
                if let Some(&v) = words.get(wi) {
                    if v == *cnt {
                        diff += 1;
                    }
                    *cnt -= 1;
                    if v == *cnt {
                        diff -= 1;
                    }
                }
                // dbg!(&hash, diff);
                let wj = &s[j..j + m];
                let cnt = hash.entry(wj).or_insert(0);
                if let Some(&v) = words.get(wj) {
                    if v == *cnt {
                        diff += 1;
                    }
                    *cnt += 1;
                    if v == *cnt {
                        diff -= 1;
                    }
                }
                // dbg!(&hash, diff);
                if diff == 0 {
                    ans.push(i + m);
                }
            }
        }

        ans.into_iter().map(|v| v as i32).collect()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "barfoothefoobarman"\n["foo","bar"]\n
// @lcpr case=end

// @lcpr case=start
// "wordgoodgoodgoodbestword"\n["word","good","best","word"]\n
// @lcpr case=end

// @lcpr case=start
// "barfoofoobarthefoobarman"\n["bar","foo","the"]\n
// @lcpr case=end

 */

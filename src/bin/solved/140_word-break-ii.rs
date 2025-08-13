/*
 * @lc app=leetcode.cn id=140 lang=rust
 * @lcpr version=30204
 *
 * [140] 单词拆分 II
 */

use leetcode_rs::str_vec;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::word_break(
            "catsanddog".to_string(),
            str_vec!["cat", "cats", "and", "sand", "dog"]
        ),
        str_vec!["cat sand dog", "cats and dog"]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        // 字典树
        #[derive(Clone)]
        struct Trie {
            children: Vec<Option<Box<Trie>>>,
            is_end: bool,
        }
        impl Trie {
            fn new() -> Self {
                Self {
                    children: vec![None; 26],
                    is_end: false,
                }
            }
            fn insert(&mut self, word: &[u8]) {
                if word.len() == 0 {
                    self.is_end = true;
                    return;
                }
                let idx = (word[0] - b'a') as usize;
                if self.children[idx].is_none() {
                    self.children[idx] = Some(Box::new(Trie::new()));
                }
                self.children[idx].as_mut().unwrap().insert(&word[1..]);
            }
            // 判断text的前缀是否在树中，返回匹配的长度
            fn search(&self, text: &[u8], prefix_len: usize) -> Vec<usize> {
                let Some(&ch) = text.first() else {
                    return if self.is_end {
                        vec![prefix_len]
                    } else {
                        vec![]
                    };
                };
                let mut ret = vec![];
                if prefix_len > 0 && self.is_end {
                    ret.push(prefix_len);
                }
                let idx = (ch - b'a') as usize;
                if let Some(child) = &self.children[idx] {
                    ret.extend(child.search(&text[1..], prefix_len + 1));
                };

                ret
            }
        }

        let mut trie = Trie::new();
        for word in word_dict {
            trie.insert(word.as_bytes());
        }

        fn dfs(text: &[u8], prefix: Vec<u8>, i: usize, ans: &mut Vec<Vec<u8>>, trie: &Trie) {
            if i == text.len() {
                ans.push(prefix);
                return;
            }
            for len in trie.search(&text[i..], 0) {
                let mut prefix = prefix.clone();
                prefix.push(b' ');
                prefix.extend_from_slice(&text[i..i + len]);
                dfs(text, prefix, i + len, ans, trie);
            }
        }

        let mut ans = vec![];
        dfs(s.as_bytes(), vec![], 0, &mut ans, &trie);
        let ans = ans
            .into_iter()
            .map(|v| unsafe { String::from_utf8_unchecked(v[1..].to_vec()) })
            .collect::<Vec<_>>();

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// "catsanddog"\n["cat","cats","and","sand","dog"]\n
// @lcpr case=end

// @lcpr case=start
// "pineapplepenapple"\n["apple","pen","applepen","pine","pineapple"]\n
// @lcpr case=end

// @lcpr case=start
// "catsandog"\n["cats","dog","sand","and","cat"]\n
// @lcpr case=end

 */

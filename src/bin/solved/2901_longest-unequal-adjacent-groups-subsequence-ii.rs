/*
 * @lc app=leetcode.cn id=2901 lang=rust
 * @lcpr version=30204
 *
 * [2901] 最长相邻不相等子序列 II
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::get_words_in_longest_subsequence(
            vec!["bab".to_string(), "dab".to_string(), "cab".to_string()],
            vec![1, 2, 2]
        ),
        vec!["bab".to_string(), "dab".to_string()]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut f = (0..n).map(|i| (1, i)).collect::<Vec<_>>();
        let ok = |w1: &str, w2: &str| {
            let (w1, w2) = (w1.as_bytes(), w2.as_bytes());
            if w1.len() != w2.len() {
                return false;
            }
            let mut diff = 0;
            for i in 0..w1.len() {
                if w1[i] != w2[i] {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
            }
            true
        };
        let (mut max_i, mut max_len) = (0, 1);
        for i in 1..n {
            for j in 0..i {
                if groups[i] != groups[j] && ok(&words[i], &words[j]) && f[j].0 >= f[i].0 {
                    f[i] = (f[j].0 + 1, j);
                    if f[i].0 > max_len {
                        max_len = f[i].0;
                        max_i = i;
                    }
                }
            }
        }

        let mut ans = vec![];
        let mut i = max_i;
        loop {
            ans.push(words[i].clone());
            if f[i].1 == i {
                break;
            }
            i = f[i].1;
        }
        ans.reverse();

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 3\n["bab","dab","cab"]\n[1,2,2]\n
// @lcpr case=end

// @lcpr case=start
// 4\n["a","b","c","d"]\n[1,2,3,4]\n
// @lcpr case=end

 */

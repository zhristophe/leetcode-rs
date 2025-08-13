/*
 * @lc app=leetcode.cn id=3529 lang=rust
 * @lcpr version=30204
 *
 * [3529] 统计水平子串和垂直子串重叠格子的数目
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::count_cells(
            vec2d![
                ['a', 'a', 'c', 'c'],
                ['b', 'b', 'b', 'c'],
                ['a', 'a', 'b', 'a'],
                ['c', 'a', 'a', 'c'],
                ['a', 'a', 'b', 'a']
            ],
            "abaca".to_string()
        ),
        1
    );
    assert_eq!(
        Solution::count_cells(
            vec2d![
                ['c', 'a', 'a', 'a'],
                ['a', 'a', 'b', 'a'],
                ['b', 'b', 'a', 'a'],
                ['a', 'a', 'b', 'a']
            ],
            "aba".to_string()
        ),
        4
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
        // 拉直了以后统计pattern出现的位置
        let m = grid.len();
        let n = grid[0].len();
        let mut s1 = vec![b'a'; m * n];
        let mut s2 = vec![b'a'; m * n];

        // kmp算法
        fn lps(pattern: &[u8]) -> Vec<usize> {
            let mut lps = vec![0; pattern.len()];
            let (mut len, mut i) = (0, 1);
            while i < pattern.len() {
                if pattern[i] == pattern[len] {
                    len += 1;
                    lps[i] = len;
                    i += 1;
                } else {
                    if len != 0 {
                        len = lps[len - 1];
                    } else {
                        lps[i] = 0;
                        i += 1;
                    }
                }
            }
            lps
        }
        fn kmp(text: &[u8], pattern: &[u8]) -> Vec<usize> {
            let lps = lps(pattern);
            let (mut i, mut j) = (0, 0);
            let mut ans = vec![];

            while i < text.len() {
                if pattern[j] == text[i] {
                    i += 1;
                    j += 1;
                    if j == pattern.len() {
                        ans.push(i - j);
                        j = lps[j - 1];
                    }
                } else if j > 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
            ans
        }

        for i in 0..m {
            for j in 0..n {
                s1[i * n + j] = grid[i][j] as u8;
                s2[j * m + i] = grid[i][j] as u8;
            }
        }
        let mut cnt = vec![vec![0; n]; m];
        let pattern = pattern.as_bytes();
        let mut i = 0;
        for j in kmp(&s1, &pattern) {
            for k in j.max(i)..j + pattern.len() {
                cnt[k / n][k % n] += 1;
            }
            i = j + pattern.len();
        }
        let mut ans = 0;
        i = 0;
        for j in kmp(&s2, &pattern) {
            for k in j.max(i)..j + pattern.len() {
                cnt[k % m][k / m] += 1;
                if cnt[k % m][k / m] >= 2 {
                    ans += 1;
                }
            }
            i = j + pattern.len();
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [["a","a","c","c"],["b","b","b","c"],["a","a","b","a"],["c","a","a","c"],["a","a","b","a"]]\n"abaca"\n
// @lcpr case=end

// @lcpr case=start
// [["c","a","a","a"],["a","a","b","a"],["b","b","a","a"],["a","a","b","a"]]\n"aba"\n
// @lcpr case=end

// @lcpr case=start
// [["a"]]\n"a"\n
// @lcpr case=end

 */

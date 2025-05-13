/*
 * @lc app=leetcode.cn id=2075 lang=rust
 * @lcpr version=30204
 *
 * [2075] 解码斜向换位密码
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::decode_ciphertext("ch   ie   pr".to_string(), 3),
        "cipher".to_string()
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let cols = encoded_text.len() / rows;
        let chars = encoded_text.chars().collect::<Vec<_>>();
        let mut matrix = Vec::new();
        for i in 0..rows {
            matrix.push(chars[i * cols..(i + 1) * cols].to_owned());
        }
        let mut ans = Vec::new();
        for j in 0..cols {
            for i in 0..rows.min(cols - j) {
                ans.push(matrix[i][j + i]);
            }
        }
        let ans = ans.iter().collect::<String>();
        ans.trim_end().to_string()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "ch   ie   pr"\n3\n
// @lcpr case=end

// @lcpr case=start
// "iveo    eed   l te   olc"\n4\n
// @lcpr case=end

// @lcpr case=start
// "coding"\n1\n
// @lcpr case=end

// @lcpr case=start
// " b  ac"\n2\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=2545 lang=rust
 * @lcpr version=30204
 *
 * [2545] 根据第 K 场考试的分数排序
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::sort_the_students(vec2d![[10, 6, 9, 1], [7, 5, 11, 2], [4, 8, 3, 15]], 2),
        vec2d![[7, 5, 11, 2], [10, 6, 9, 1], [4, 8, 3, 15]]
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score;
        score.sort_by_key(|a| -a[k as usize]);
        score
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[10,6,9,1],[7,5,11,2],[4,8,3,15]]\n2\n
// @lcpr case=end

// @lcpr case=start
// [[3,4],[5,6]]\n0\n
// @lcpr case=end

 */

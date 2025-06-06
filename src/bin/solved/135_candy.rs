/*
 * @lc app=leetcode.cn id=135 lang=rust
 * @lcpr version=30204
 *
 * [135] 分发糖果
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    assert_eq!(Solution::candy(vec![0]), 1);
    assert_eq!(Solution::candy(vec![0, 1]), 3);
    assert_eq!(Solution::candy(vec![1, 0]), 3);
    assert_eq!(Solution::candy(vec![0, 1, 0]), 4);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        // 这是hard吗？
        // 检测极小点，处理两边的糖果分发。线性复杂度
        let mut candy = vec![1; ratings.len()];
        let mut i = 0;
        while i < ratings.len() {
            if (i == 0 || ratings[i] <= ratings[i - 1])
                && (i == ratings.len() - 1 || ratings[i] <= ratings[i + 1])
            {
                // 极小点
                candy[i] = 1;
                let mut j = i;
                while j > 0 && ratings[j - 1] > ratings[j] {
                    candy[j - 1] = candy[j - 1].max(candy[j] + 1);
                    j -= 1;
                }
                while i < ratings.len() - 1 && ratings[i + 1] > ratings[i] {
                    candy[i + 1] = candy[i] + 1;
                    i += 1;
                }
            }
            i += 1;
        }
        // dbg!(&candy);
        candy.into_iter().sum()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,0,2]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,2]\n
// @lcpr case=end

 */

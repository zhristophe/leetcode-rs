/*
 * @lc app=leetcode.cn id=2256 lang=rust
 * @lcpr version=30204
 *
 * [2256] 最小平均差
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]),
        3
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        // 注意：加法会溢出
        let n = nums.len();
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let sum: i64 = nums.iter().sum();
        let mut min_dis = i64::MAX;
        let mut min_idx = 0;
        let mut left_sum = 0;
        for i in 0..n {
            left_sum += nums[i];
            let left_avg = left_sum / (i as i64 + 1);
            let right_avg = if i == n - 1 {
                0
            } else {
                (sum - left_sum) / (n as i64 - i as i64 - 1)
            };
            let dis = (left_avg - right_avg).abs();
            if dis < min_dis {
                min_dis = dis;
                min_idx = i;
            }
        }
        min_idx as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,5,3,9,5,3]\n
// @lcpr case=end

// @lcpr case=start
// [0]\n
// @lcpr case=end

 */

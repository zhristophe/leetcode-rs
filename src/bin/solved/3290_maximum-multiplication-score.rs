/*
 * @lc app=leetcode.cn id=3290 lang=rust
 * @lcpr version=30204
 *
 * [3290] 最高乘法得分
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_score(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7]),
        26
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let a = a.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let b = b.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut pre_max = vec![0; b.len()];
        pre_max[0] = a[0] * b[0];
        for j in 1..b.len() {
            pre_max[j] = pre_max[j - 1].max(a[0] * b[j]);
        }
        for i in 1..a.len() {
            let mut next_pre_max = vec![0; b.len()];
            next_pre_max[i] = pre_max[i - 1] + a[i] * b[i];
            for j in i + 1..b.len() {
                next_pre_max[j] = next_pre_max[j - 1].max(pre_max[j - 1] + a[i] * b[j]);
            }
            pre_max = next_pre_max;
        }

        pre_max.last().unwrap().clone()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,2,5,6]\n[2,-6,4,-5,-3,2,-7]\n
// @lcpr case=end

// @lcpr case=start
// [-1,4,5,-2]\n[-5,-1,-3,-2,-4]\n
// @lcpr case=end

 */

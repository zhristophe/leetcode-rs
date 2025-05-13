/*
 * @lc app=leetcode.cn id=3041 lang=rust
 * @lcpr version=30204
 *
 * [3041] 修改数组后最大化数组中的连续元素数目
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
    assert_eq!(Solution::max_selected_elements(vec![1, 4, 7, 10]), 1);
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut nums = nums;
        nums.sort();

        let mut dp = vec![vec![1, 1]; nums.len()];
        let mut hash = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            hash.entry(*n).or_insert(Vec::new()).push(i);
        }
        for i in 1..nums.len() {
            let ni = nums[i];
            // dbg!(
            //     ni,
            //     hash.get(&(ni - 2)),
            //     hash.get(&(ni - 1)),
            //     hash.get(&(ni - 0))
            // );
            if let Some(js) = hash.get(&(ni - 2)) {
                for &j in js {
                    if j >= i {
                        break;
                    }
                    dp[i][0] = dp[i][0].max(dp[j][1] + 1);
                }
            }
            if let Some(js) = hash.get(&(ni - 1)) {
                for &j in js {
                    if j >= i {
                        break;
                    }
                    dp[i][0] = dp[i][0].max(dp[j][0] + 1);
                    dp[i][1] = dp[i][1].max(dp[j][1] + 1);
                }
            }
            if let Some(js) = hash.get(&(ni)) {
                for &j in js {
                    if j >= i {
                        break;
                    }
                    dp[i][1] = dp[i][1].max(dp[j][0] + 1);
                }
            }
            // dbg!(&dp[i]);
        }
        // dbg!(&dp);

        dp.iter()
            .map(|v| v.iter())
            .flatten()
            .max()
            .unwrap()
            .to_owned()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,1,5,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,4,7,10]\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=2610 lang=rust
 * @lcpr version=30204
 *
 * [2610] 转换二维数组
 */

// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cnt = HashMap::new();
        let mut ans = vec![];
        for num in nums {
            let cnt = cnt.entry(num).or_insert(0);
            *cnt += 1;
            if ans.len() < *cnt {
                ans.push(vec![]);
            }
            ans[*cnt - 1].push(num);
        }

        ans
    }
}
// @lc code=end
use leetcode_rs::vec2d;
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
        vec2d![[1, 3, 4, 2], [1, 3], [1]]
    );
}
/*
// @lcpr case=start
// [1,3,4,1,2,3,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4]\n
// @lcpr case=end

 */

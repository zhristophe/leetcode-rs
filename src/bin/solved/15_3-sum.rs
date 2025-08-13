/*
 * @lc app=leetcode.cn id=15 lang=rust
 * @lcpr version=30204
 *
 * [15] 三数之和
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec2d![[-1, -1, 2], [-1, 0, 1]]
    );
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec2d![[0, 0, 0]]);
    assert_eq!(Solution::three_sum(vec![0, 0, -1, 1]), vec2d![[-1, 0, 1]]);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 枚举第一个数，后两个数使用双指针
        let mut ans = vec![];
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        for k in 0..n - 2 {
            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }
            let mut i = k + 1;
            let mut j = n - 1;
            loop {
                if i >= j {
                    break;
                }
                if i > k + 1 && nums[i] == nums[i - 1] {
                    i += 1;
                    continue;
                }
                let tgt = -nums[k] - nums[i];
                while j > i && nums[j] > tgt {
                    j -= 1;
                }
                if j == i {
                    break;
                }
                if nums[j] == tgt {
                    ans.push(vec![nums[k], nums[i], nums[j]]);
                }
                i += 1;
            }
        }

        ans
    }
    #[allow(dead_code)]
    pub fn three_sum_slow(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 枚举前两个数然后二分搜索
        use std::collections::HashSet;
        let mut ans = HashSet::new();
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if nums[i] + nums[j] + nums[j] > 0 {
                    break;
                }
                if !ans.contains(&(nums[i], nums[j])) {
                    if let Ok(_) = nums[j + 1..].binary_search(&(-nums[i] - nums[j])) {
                        ans.insert((nums[i], nums[j]));
                    }
                }
            }
        }

        ans.into_iter().map(|(a, b)| vec![a, b, -a - b]).collect()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [-1,0,1,2,-1,-4]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [0,0,0]\n
// @lcpr case=end

 */

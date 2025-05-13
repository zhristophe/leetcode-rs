/*
 * @lc app=leetcode.cn id=4 lang=rust
 * @lcpr version=30204
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lcpr-template-start
struct Solution;
fn main() {}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn median(nums: &[i32]) -> f64 {
            match nums.len() {
                0 => 0.0,
                1 => nums[0] as f64,
                _ => {
                    let mid = nums.len() / 2;
                    if nums.len() % 2 == 0 {
                        (nums[mid - 1] + nums[mid]) as f64 / 2.0
                    } else {
                        nums[mid] as f64
                    }
                }
            }
        }
        fn find_median(nums1: &[i32], nums2: &[i32]) -> f64 {
            let med1 = median(nums1);
            let med2 = median(nums2);
            if med1 == med2 {
                return med1;
            }
            0.0
        }

        0.0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3]\n[2]\n
// @lcpr case=end

// @lcpr case=start
// [1,2]\n[3,4]\n
// @lcpr case=end

 */

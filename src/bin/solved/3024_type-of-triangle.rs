/*
 * @lc app=leetcode.cn id=3024 lang=rust
 * @lcpr version=30204
 *
 * [3024] 三角形类型
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::triangle_type(vec![3, 3, 3]),
        "equilateral".to_owned()
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable();
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        if a + b <= c {
            return "none".to_owned();
        }
        let mut ans = 0;
        if a == b {
            ans += 1;
        }
        if a == c {
            ans += 1;
        }
        if b == c {
            ans += 1;
        }

        match ans {
            0 => "scalene",
            1 => "isosceles",
            3 => "equilateral",
            _ => unreachable!(),
        }
        .into()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,3,3]\n
// @lcpr case=end

// @lcpr case=start
// [3,4,5]\n
// @lcpr case=end

 */

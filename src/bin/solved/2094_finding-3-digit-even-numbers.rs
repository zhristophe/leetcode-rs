/*
 * @lc app=leetcode.cn id=2094 lang=rust
 * @lcpr version=30204
 *
 * [2094] 找出 3 位偶数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_even_numbers(vec![2, 1, 3, 0]),
        vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
    );
    assert_eq!(Solution::find_even_numbers(vec![0, 0, 2]), vec![200]);
    assert_eq!(Solution::find_even_numbers(vec![0, 0, 0]), vec![]);
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut ds = [0; 10];
        digits.iter().for_each(|d| ds[*d as usize] += 1);
        let mut ans = HashSet::new();
        for i in 1..=9 {
            if ds[i] == 0 {
                continue;
            }
            ds[i] -= 1;
            for j in 0..=9 {
                if ds[j] == 0 {
                    continue;
                }
                ds[j] -= 1;
                for k in 0..=9 {
                    if k % 2 == 1 {
                        continue;
                    }
                    if ds[k] > 0 {
                        ans.insert((100 * i + 10 * j + k) as i32);
                    }
                }
                ds[j] += 1;
            }
            ds[i] += 1;
        }
        let mut ans = ans.into_iter().collect::<Vec<_>>();
        ans.sort();

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,1,3,0]\n
// @lcpr case=end

// @lcpr case=start
// [2,2,8,8,2]\n
// @lcpr case=end

// @lcpr case=start
// [3,7,5]\n
// @lcpr case=end

 */

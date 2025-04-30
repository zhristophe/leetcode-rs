/*
 * @lc app=leetcode id=2967 lang=rust
 *
 * [2967] Minimum Cost to Make Array Equalindromic
 */

// @lc code=start
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(|i| i as i64).collect::<Vec<_>>();
        nums.sort();
        let n = nums.len();
        let mid = if n % 2 == 0 {
            (nums[n / 2 - 1] + nums[n / 2]) / 2
        } else {
            nums[n / 2]
        };
        let get_palindromic = |num: i64| -> i64 {
            let text: String = num.to_string();
            let center_index = text.len() / 2;
            let prefix = text[..center_index].to_owned();
            let center = if text.len() % 2 == 0 {
                "".to_owned()
            } else {
                text[center_index..center_index + 1].to_owned()
            };
            let suffix = prefix.chars().rev().collect::<String>();
            let ret = prefix + &center + &suffix;
            ret.parse().unwrap()
        };
        let y0 = get_palindromic(mid);
        let mid_n_digits = mid.to_string().len();
        let y1 = if y0 == mid {
            mid
        } else if y0 < mid {
            let p = mid_n_digits / 2;
            mid + 10i64.pow(p as u32)
        } else {
            let p = mid_n_digits / 2;
            (10i64.pow(mid_n_digits as u32 - 1) - 1).max(mid - 10i64.pow(p as u32))
        };
        let cost = |y: i64| nums.iter().fold(0i64, |acc, v| acc + (y - v).abs() as i64);
        let cost0 = cost(y0);
        // dbg!(y1);
        let y1 = get_palindromic(y1);
        // dbg!(y0, y1, cost0);
        if y1 == y0 {
            cost0
        } else {
            let cost1 = cost(y1);
            cost0.min(cost1)
        }
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 4, 5]), 6);
    assert_eq!(Solution::minimum_cost(vec![10, 12, 13, 14, 15]), 11);
    assert_eq!(Solution::minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    assert_eq!(Solution::minimum_cost(vec![900, 1000, 1000]), 101);
}

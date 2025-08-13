/*
 * @lc app=leetcode.cn id=3569 lang=rust
 * @lcpr version=30204
 *
 * [3569] 分割数组后不同质数的最大数目
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::maximum_count(vec![2, 1, 3, 1, 2], vec2d![[1, 2], [3, 3]]),
        vec![3, 4]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn maximum_count(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 先计算质数，nlogn
        // 对于每个质数，维护其坐标
        // 然后查询，mlogn
        let mut nums = nums;
        let mut is_prime = vec![true; 100_000 + 10];
        let mut primes = vec![];
        for i in 2..=100_000 {
            let is = primes.iter().all(|p| i % p != 0);
            if is {
                primes.push(i);
            } else {
                is_prime[i] = false;
            }
        }
        // 维护素数坐标
        use std::collections::BTreeSet;
        let mut primes_pos = vec![BTreeSet::new(); primes.len()];
        for (i, &n) in nums.iter().enumerate() {
            let n = n as usize;
            if is_prime[n] {
                primes_pos[n].insert(i);
            }
        }
        let mut ans = vec![];
        for query in queries {
            let idx = query[0] as usize;
            let val = query[1];
            let old = nums[idx];
            if is_prime[old as usize] {
                primes_pos[old as usize].remove(&idx);
            }
            if is_prime[val as usize] {
                primes_pos[val as usize].insert(idx);
            }
        }

        todo!()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,1,3,1,2]\n[[1,2],[3,3]]\n
// @lcpr case=end

// @lcpr case=start
// [2,1,4]\n[[0,1]]\n
// @lcpr case=end

 */

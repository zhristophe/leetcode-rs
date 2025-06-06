/*
 * @lc app=leetcode.cn id=2354 lang=rust
 * @lcpr version=30204
 *
 * [2354] 优质数对的数目
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
    assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1], 60), 0);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        // 两个一位数的与或之和等于它们的和
        // 即求一位数和大于等于k的数对数目
        let mut cnt = vec![std::collections::HashSet::new(); 32]; // 存1个数为i的个数
        for num in nums {
            cnt[num.count_ones() as usize].insert(num);
        }
        let cnt = cnt.into_iter().map(|s| s.len()).collect::<Vec<_>>();
        let mut sum = cnt.clone();
        for i in (0..32).rev().skip(1) {
            sum[i] += sum[i + 1];
        }
        let mut ans = 0;
        for i in 0..32 {
            let sum = if i >= k {
                sum[0]
            } else if k - i >= 32 {
                0
            } else {
                sum[(k - i) as usize]
            };
            ans += sum * cnt[i as usize];
        }

        ans as i64
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,1]\n3\n
// @lcpr case=end

// @lcpr case=start
// [5,1,1]\n10\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=2191 lang=rust
 * @lcpr version=30204
 *
 * [2191] 将杂乱无章的数字排序
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
        vec![338, 38, 991]
    );
    assert_eq!(
        Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
        vec![123, 456, 789]
    );
    assert_eq!(
        Solution::sort_jumbled(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        ),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut nums2: Vec<_> = nums
            .iter()
            .enumerate()
            .map(|(index, n)| {
                let mut digits = vec![];
                let mut n = *n;
                if n == 0 {
                    digits.push(0);
                }
                while n > 0 {
                    digits.push(n % 10);
                    n /= 10;
                }
                for digit in digits.into_iter().rev() {
                    n *= 10;
                    n += mapping[digit as usize];
                }

                (n, index)
            })
            .collect();
        nums2.sort_unstable_by_key(|n| n.0);
        nums2.into_iter().map(|(_, index)| nums[index]).collect()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [8,9,4,0,2,1,3,5,7,6]\n[991,338,38]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,2,3,4,5,6,7,8,9]\n[789,456,123]\n
// @lcpr case=end

 */

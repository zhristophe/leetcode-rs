/*
 * @lc app=leetcode.cn id=1906 lang=rust
 * @lcpr version=30204
 *
 * [1906] 查询差绝对值的最小值
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::min_difference(vec![1, 3, 4, 8], vec2d![[0, 1], [1, 2], [2, 3], [0, 3]]),
        vec![2, 1, 4, 1]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pre = vec![[0; 101]; nums.len()];
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 {
                pre[i] = pre[i - 1];
            }
            pre[i][num as usize] += 1;
        }
        // for pre in &pre {
        //     for i in 1..=100 {
        //         if pre[i] > 0 {
        //             print!("{}: {} ", i, pre[i])
        //         }
        //     }
        //     println!();
        // }
        let mut ans = vec![];
        for q in queries {
            let mut cnt = pre[q[1] as usize];
            if q[0] != 0 {
                for i in 1..=100 {
                    cnt[i] -= pre[q[0] as usize - 1][i];
                }
            }
            let mut min_diff = i32::MAX;
            let next_num = |mut i| {
                while i <= 100 && cnt[i] == 0 {
                    i += 1;
                }
                i
            };
            let mut prev = next_num(0);
            loop {
                let next = next_num(prev + 1);
                if next > 100 {
                    break;
                }
                min_diff = min_diff.min(next as i32 - prev as i32);
                prev = next;
            }
            ans.push(if min_diff == i32::MAX { -1 } else { min_diff });
        }

        ans
    }

    #[allow(dead_code)]
    pub fn min_difference_slow(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 前缀和更合适
        struct SegTree {
            n: usize,
            data: Vec<[bool; 101]>,
        }
        impl SegTree {
            fn new(nums: &[i32]) -> Self {
                let mut n = 1;
                while n < nums.len() {
                    n <<= 1;
                }
                let mut data = vec![[false; 101]; n << 1];
                for (index, &num) in nums.iter().enumerate() {
                    data[n as usize + index] = [false; 101];
                    data[n as usize + index][num as usize] = true;
                }
                for i in (1..n).rev() {
                    for j in 1..=100 {
                        data[i][j] = data[i * 2][j] || data[i * 2 + 1][j]
                    }
                }
                Self { n, data }
            }
            fn query(&self, mut l: usize, mut r: usize) -> [bool; 101] {
                l += self.n;
                r += self.n;
                let mut rl = [false; 101];
                let mut rr = [false; 101];
                while l <= r {
                    if l % 2 == 1 {
                        for i in 1..=100 {
                            rl[i] |= self.data[l][i];
                        }
                        l += 1;
                    }
                    if r % 2 == 0 {
                        for i in 1..=100 {
                            rr[i] |= self.data[r][i];
                        }
                        r -= 1;
                    }
                    l /= 2;
                    r /= 2;
                }
                for i in 1..=100 {
                    rl[i] |= rr[i];
                }
                rl
            }
        }
        let st = SegTree::new(&nums);
        let mut ans = vec![];
        for q in queries {
            let nums = st.query(q[0] as usize, q[1] as usize);
            let mut minv = 101;
            let next_num = |mut i| {
                while i <= 100 && nums[i] == false {
                    i += 1;
                }
                i
            };
            let mut prev = next_num(0);
            loop {
                let next = next_num(prev + 1);
                if next > 100 {
                    break;
                }
                minv = minv.min(next as i32 - prev as i32);
                prev = next;
            }
            ans.push(if minv == 101 { -1 } else { minv });
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,4,8]\n[[0,1],[1,2],[2,3],[0,3]]\n
// @lcpr case=end

// @lcpr case=start
// [4,5,2,2,7,10]\n[[2,3],[0,2],[0,5],[3,5]]\n
// @lcpr case=end

 */

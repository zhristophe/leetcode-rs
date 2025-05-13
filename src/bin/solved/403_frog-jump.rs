/*
 * @lc app=leetcode.cn id=403 lang=rust
 * @lcpr version=30204
 *
 * [403] 青蛙过河
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
    assert_eq!(Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
    assert_eq!(Solution::can_cross(vec![0, 2]), false);
    assert_eq!(Solution::can_cross(vec![0, 1, 3, 6, 10, 15]), true);
    assert_eq!(
        Solution::can_cross(vec![0, 1, 3, 6, 10, 15, 19, 22, 24, 25]),
        true
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        struct Data {
            failed: HashSet<(usize, i32)>,
            stones: Vec<i32>,
        }
        let mut data = Data {
            failed: HashSet::new(),
            stones,
        };
        fn search(i: usize, k: i32, data: &mut Data) -> bool {
            let n = data.stones.len();
            if i == n - 1 {
                return true;
            }
            if data.failed.contains(&(i, k)) {
                return false;
            }
            // 懒得二分了，直接线性检索
            let mut j = i + 1;
            let res = loop {
                if j >= n {
                    break false;
                }
                let dis = data.stones[j] - data.stones[i];
                if dis < k - 1 {
                } else if dis > k + 1 {
                    break false;
                } else {
                    if search(j, dis, data) {
                        break true;
                    }
                }
                j += 1;
            };
            if res {
                true
            } else {
                data.failed.insert((i, k));
                false
            }
        }
        search(0, 0, &mut data)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [0,1,3,5,6,8,12,17]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,2,3,4,8,9,11]\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=1298 lang=rust
 * @lcpr version=30204
 *
 * [1298] 你能从盒子里获得的最大糖果数
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_candies(
            vec![1, 0, 1, 0],
            vec![7, 5, 4, 100],
            vec2d![[], [], [1], []],
            vec2d![[1, 2], [3], [], []],
            vec![0]
        ),
        16
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();
        let mut open = status;
        let mut free = vec![false; n];
        for b in initial_boxes {
            free[b as usize] = true;
        }
        let mut q = std::collections::VecDeque::new();
        enum Op {
            Open,
            Free,
        }
        for i in 0..n {
            if open[i] == 1 {
                q.push_back((i, Op::Open));
            }
            if free[i] {
                q.push_back((i, Op::Free));
            }
        }
        let mut ans = 0;
        let mut candies = candies;
        while let Some((i, _)) = q.pop_front() {
            if open[i] == 0 || !free[i] {
                continue;
            }
            ans += candies[i];
            candies[i] = 0;
            for &key in &keys[i] {
                let key = key as usize;
                if open[key] == 0 {
                    open[key] = 1;
                    q.push_back((key, Op::Open));
                }
            }
            for &inner in &contained_boxes[i] {
                let inner = inner as usize;
                if free[inner] == false {
                    free[inner] = true;
                    q.push_back((inner, Op::Free));
                }
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,0,1,0]\n[7,5,4,100]\n[[],[],[1],[]]\n[[1,2],[3],[],[]]\n[0]\n
// @lcpr case=end

// @lcpr case=start
// [1,0,0,0,0,0]\n[1,1,1,1,1,1]\n[[1,2,3,4,5],[],[],[],[],[]]\n[[1,2,3,4,5],[],[],[],[],[]]\n[0]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1]\n[100,1,100]\n[[],[0,2],[]]\n[[],[],[]]\n[1]\n
// @lcpr case=end

// @lcpr case=start
// [1]\n[100]\n[[]]\n[[]]\n[]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1]\n[2,3,2]\n[[],[],[]]\n[[],[],[]]\n[2,1,0]\n
// @lcpr case=end

 */

/*
 * @lc app=leetcode.cn id=1024 lang=rust
 * @lcpr version=30204
 *
 * [1024] 视频拼接
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::video_stitching(vec2d![[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10),
        3
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        // time < 100, 因此用它来处理
        let mut t = vec![0; time as usize + 1];
        for clip in clips.iter() {
            for i in clip[0] as usize..=clip[1].min(time) as usize {
                t[i] = t[i].max(clip[1]);
            }
        }
        let mut i = 0;
        let mut ans = 0;
        loop {
            if t[i as usize] > i {
                ans += 1;
                i = t[i as usize];
                if i >= time {
                    return ans;
                }
            } else {
                break -1;
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]]\n10\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[1,2]]\n5\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]]\n9\n
// @lcpr case=end

 */

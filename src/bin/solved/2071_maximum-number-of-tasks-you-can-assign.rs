/*
 * @lc app=leetcode.cn id=2071 lang=rust
 * @lcpr version=30204
 *
 * [2071] 你可以安排的最多任务数目
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1),
        3
    );
    assert_eq!(
        Solution::max_task_assign(vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10),
        2
    );
    assert_eq!(
        Solution::max_task_assign(vec![5, 9, 8, 5, 9], vec![1, 6, 4, 2, 6], 1, 5),
        3
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        // 二分法搜索
        // 优化方案：收集所有吃药可以完成任务的工人，然后如果最大值可以完成任务
        // 那么就用最大值
        let mut tasks = tasks;
        let mut workers = workers;
        tasks.sort_unstable();
        workers.sort_unstable();

        let able = |mid| {
            let mut pills = pills;
            let mut worker_queue = VecDeque::new();
            let mut workers = workers.iter().rev().take(mid).cloned().peekable();
            for task in tasks.iter().take(mid).rev() {
                while let Some(worker) = workers.peek() {
                    if worker + strength >= *task {
                        worker_queue.push_front(workers.next().unwrap());
                    } else {
                        break;
                    }
                }
                if worker_queue.is_empty() {
                    return false;
                } else {
                    if worker_queue.back().unwrap() >= task {
                        worker_queue.pop_back();
                    } else if pills == 0 {
                        return false;
                    } else {
                        pills -= 1;
                        worker_queue.pop_front();
                    }
                }
            }

            true
        };

        let mut l = 0;
        let mut r = tasks.len().min(workers.len());
        while l < r {
            let mid = (l + r + 1) / 2;
            if able(mid) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        l as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,2,1]\n[0,3,3]\n1\n1\n
// @lcpr case=end

// @lcpr case=start
// [5,4]\n[0,0,0]\n1\n5\n
// @lcpr case=end

// @lcpr case=start
// [10,15,30]\n[0,10,10,10,10]\n3\n10\n
// @lcpr case=end

// @lcpr case=start
// [5,9,8,5,9]\n[1,6,4,2,6]\n1\n5\n
// @lcpr case=end

 */

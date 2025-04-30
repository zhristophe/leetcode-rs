use std::{cmp::Ordering, collections::BinaryHeap, iter::Rev, num::Wrapping};

struct Solution;

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        #[derive(Debug, PartialEq, Eq)]
        enum State {
            Right,
            Pick,
            Left,
            Put,
        }
        #[derive(Debug)]
        struct Worker {
            time: i32,
            idx: usize,
            right: i32,
            left: i32,
            pick: i32,
            put: i32,
            state: State,
            left_time: i32,
        }
        impl Ord for Worker {
            fn cmp(&self, other: &Self) -> Ordering {
                let o1 = self.time.cmp(&other.time);
                match o1 {
                    Ordering::Equal => other.idx.cmp(&self.idx),
                    _ => o1,
                }
            }
        }
        impl PartialOrd for Worker {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl PartialEq for Worker {
            fn eq(&self, other: &Self) -> bool {
                self.time == other.time && self.idx == other.idx
            }
        }
        impl Eq for Worker {}
        let mut left_workers = BinaryHeap::new();
        // let mut right_workers = BinaryHeap::new();
        let workers: Vec<_> = time
            .into_iter()
            .enumerate()
            .map(|(idx, time)| Worker {
                time: time[0] + time[2],
                idx,
                right: time[0],
                left: time[2],
                pick: time[1],
                put: time[3],
                state: State::Right,
                left_time: time[0] + time[2],
            })
            .collect();
        for worker in &workers {
            left_workers.push(worker);
        }
        let mut time_seq = BinaryHeap::new();
        let mut prev_time = 0;
        time_seq.push(left_workers.peek().unwrap().right);
        let mut k = k;

        while let Some(time) = time_seq.pop() {
            for worker in &workers {}
        }

        0
    }
}

fn main() {
    use leetcode_rs::vec2d;
    Solution::find_crossing_time(
        1,
        3,
        vec2d![[1, 1, 2, 1], [1, 1, 3, 1], [1, 1, 4, 1], [1, 1, 4, 1]],
    );
}

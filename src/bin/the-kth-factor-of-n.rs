use std::vec;

struct Solution;
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        // 只枚举根号n内的
        let mut i = 1;
        let mut factors = Vec::new();
        while i * i < n {
            if n % i == 0 {
                factors.push(i);
                if factors.len() == k as usize {
                    return i;
                }
            }
            i += 1;
        }
        let idx = k - factors.len() as i32 - 1;
        if i * i == n {
            factors.push(i);
        }
        match factors.iter().nth_back(idx as usize) {
            Some(x) => n / x,
            None => -1,
        }
    }
    #[allow(dead_code)]
    fn kth_factor_o_n(n: i32, k: i32) -> i32 {
        let mut k = k;
        for i in 1..=n {
            if n % i == 0 {
                k -= 1;
                if k == 0 {
                    return i;
                }
            }
        }
        -1
    }
    #[allow(dead_code)]
    fn kth_factor_tmp(n: i32, k: i32) -> i32 {
        let mut factors = vec![];
        let mut cnt = 0;
        let mut n = n;
        while n % 2 == 0 {
            // n > 1
            n /= 2;
            cnt += 1;
        }
        factors.push((2, cnt));
        let mut i = 3;
        while i * i < n {
            if n % i == 0 {
                let mut cnt = 0;
                while n % i == 0 {
                    n /= i;
                    cnt += 1;
                }
                factors.push((i, cnt));
            }
            i += 2;
        }
        if n > 1 {
            factors.push((n, 1));
        }
        // 计算前缀和
        let prefix: Vec<_> = factors
            .iter()
            .scan(0, |acc, (_, exp)| {
                *acc += exp;
                Some(*acc)
            })
            .collect();
        let idx = Self::find_first_bigger_or_equal(&prefix, k);
        match idx {
            Some(idx) => factors[idx].0,
            None => -1,
        }
    }
    fn find_first_bigger_or_equal(arr: &[i32], tgt: i32) -> Option<usize> {
        let (mut left, mut right) = (0, arr.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < tgt {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left < arr.len() {
            return Some(left);
        }

        None
    }
}

fn main() {
    let cases = [(12, 3), (7, 2), (4, 4), (1, 1), (9, 3)];
    for (n, k) in cases {
        println!("{}", Solution::kth_factor(n, k))
    }
}

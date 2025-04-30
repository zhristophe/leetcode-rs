struct Solution;
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0 && (n & 0xaaaaaaaau32 as i32) == 0
    }
    #[allow(dead_code)]
    pub fn is_power_of_four_slowly(n: i32) -> bool {
        let mut n = n;
        while n != 0 && n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

fn main() {
    println!("{:?}", Solution::is_power_of_four(16));
}

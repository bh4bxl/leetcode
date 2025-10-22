// Created by bh4bxl at 2025/10/20 15:06
// leetgo: 1.4.15
// https://leetcode.com/problems/count-primes/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut not_prime = vec![false; n as usize];
        let mut count = 0;

        for i in 2..n as usize {
            if !not_prime[i] {
                count += 1;
                let mut j = 2;
                while i * j < n as usize {
                    not_prime[(i * j) as usize] = true;
                    j += 1;
                }
            }
        }

        count
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::count_primes(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

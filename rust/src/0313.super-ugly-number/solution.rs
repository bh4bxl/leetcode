// Created by bh4bxl at 2025/12/09 13:57
// leetgo: 1.4.15
// https://leetcode.com/problems/super-ugly-number/

use std::iter;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let len = primes.len();
        let n = n as usize;
        let mut dp = vec![0i64; n as usize];
        dp[0] = 1;
        let mut idx = vec![0; len];
        let mut factor: Vec<i64> = primes.iter().map(|&p| p as i64).collect();

        for i in 1..n {
            let next = *factor.iter().min().unwrap();
            dp[i] = next;
            for j in 0..len {
                if factor[j] == next {
                    idx[j] += 1;
                    factor[j] = dp[idx[j]] * primes[j] as i64;
                }
            }
        }

        dp[n - 1] as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let primes: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::nth_super_ugly_number(n, primes).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

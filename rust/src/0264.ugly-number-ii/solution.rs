// Created by bh4bxl at 2025/11/17 22:14
// leetgo: 1.4.15
// https://leetcode.com/problems/ugly-number-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        let (mut idx2, mut idx3, mut idx5) = (0, 0, 0);
        let (mut factor2, mut factor3, mut factor5) = (2, 3, 5);

        for i in 1..=n as usize {
            dp[i] = factor2.min(factor3.min(factor5));
            if dp[i] == factor2 {
                idx2 += 1;
                factor2 = dp[idx2] * 2;
            }
            if dp[i] == factor3 {
                idx3 += 1;
                factor3 = dp[idx3] * 3;
            }
            if dp[i] == factor5 {
                idx5 += 1;
                factor5 = dp[idx5] * 5;
            }
        }

        dp[n as usize - 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::nth_ugly_number(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

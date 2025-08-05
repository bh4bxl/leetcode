// Created by bh4bxl at 2025/08/04 22:28
// leetgo: 1.4.15
// https://leetcode.com/problems/climbing-stairs/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];

        dp[0] = 1;
        dp[1] = 1;

        for i in 2..dp.len() {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n as usize]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::climb_stairs(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

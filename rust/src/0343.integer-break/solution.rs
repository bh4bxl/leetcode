// Created by bh4bxl at 2025/12/23 13:16
// leetgo: 1.4.15
// https://leetcode.com/problems/integer-break/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n + 1];

        for i in 2..=n {
            let mut max = 0;
            for j in 1..=i / 2 {
                max = max.max((dp[j] * dp[i - j]).max((j * (i - j)) as i32));
                max = max.max((dp[j] * (i - j) as i32).max(j as i32 * dp[i - j]));
            }
            dp[i] = max;
        }

        dp[n]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::integer_break(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

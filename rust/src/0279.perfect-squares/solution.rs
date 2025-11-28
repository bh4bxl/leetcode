// Created by bh4bxl at 2025/11/24 21:27
// leetgo: 1.4.15
// https://leetcode.com/problems/perfect-squares/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];

        for i in 1..=n {
            let mut res = i32::MAX;
            let mut j = 1;
            while j * j <= i {
                res = res.min(dp[(i - j * j) as usize] + 1);
                j += 1;
            }

            dp[i as usize] = res;
        }

        dp[n as usize]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_squares(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

// Created by bh4bxl at 2025/08/20 12:32
// leetgo: 1.4.15
// https://leetcode.com/problems/unique-binary-search-trees/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];

        dp[0] = 1;
        dp[1] = 1;

        for nodes in 2..=n {
            for root in 1..=nodes {
                dp[nodes as usize] += dp[root as usize - 1] * dp[(nodes - root) as usize];
            }
        }

        dp[n as usize]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_trees(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

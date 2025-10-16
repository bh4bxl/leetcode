// Created by bh4bxl at 2025/10/14 14:36
// leetgo: 1.4.15
// https://leetcode.com/problems/dungeon-game/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (height, width) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![0; width]; height];
        dp[height - 1][width - 1] = 1.max(1 - dungeon[height - 1][width - 1]);
        for i in (0..height - 1).rev() {
            dp[i][width - 1] = 1.max(dp[i + 1][width - 1] - dungeon[i][width - 1]);
        }
        for i in (0..width - 1).rev() {
            dp[height - 1][i] = 1.max(dp[height - 1][i + 1] - dungeon[height - 1][i]);
        }
        for i in (0..height - 1).rev() {
            for j in (0..width - 1).rev() {
                dp[i][j] = 1.max(dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]);
            }
        }

        dp[0][0]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let dungeon: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::calculate_minimum_hp(dungeon).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

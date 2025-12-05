// Created by bh4bxl at 2025/12/01 14:14
// leetgo: 1.4.15
// https://leetcode.com/problems/nim-game/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_win_nim(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

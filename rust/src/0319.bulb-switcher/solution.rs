// Created by bh4bxl at 2025/12/09 21:22
// leetgo: 1.4.15
// https://leetcode.com/problems/bulb-switcher/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        n.isqrt()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::bulb_switch(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

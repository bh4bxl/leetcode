// Created by bh4bxl at 2026/01/20 20:26
// leetgo: 1.4.16
// https://leetcode.com/problems/elimination-game/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut head = 1;
        let mut step = 1;
        let mut remaining = n;
        let mut left = true;

        while remaining > 1 {
            if left || remaining % 2 == 1 {
                head += step;
            }

            remaining /= 2;
            step *= 2;
            left = !left;
        }

        head
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::last_remaining(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

// Created by bh4bxl at 2025/12/15 19:45
// leetgo: 1.4.15
// https://leetcode.com/problems/power-of-three/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut m = n;

        if n == 1 {
            return true;
        }

        loop {
            if m == 3 {
                return true;
            } else if m < 3 {
                return false;
            } else {
                if m % 3 != 0 {
                    return false;
                } else {
                    m = m / 3;
                }
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_power_of_three(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

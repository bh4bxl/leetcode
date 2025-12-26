// Created by bh4bxl at 2025/12/23 12:52
// leetgo: 1.4.15
// https://leetcode.com/problems/power-of-four/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        if n == 1 {
            return true;
        }

        let mut n = n;
        for _ in 1..16 {
            if n & 0b011 != 0 {
                return false;
            }
            n >>= 2;
            if n == 1 {
                return true;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_power_of_four(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

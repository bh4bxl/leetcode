// Created by bh4bxl at 2025/11/17 22:06
// leetgo: 1.4.15
// https://leetcode.com/problems/ugly-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n <= 0 {
            return false;
        }
        if n == 2 || n == 3 || n == 5 {
            return true;
        }
        if n % 2 == 0 {
            return Self::is_ugly(n / 2);
        }
        if n % 3 == 0 {
            return Self::is_ugly(n / 3);
        }
        if n % 5 == 0 {
            return Self::is_ugly(n / 5);
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_ugly(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

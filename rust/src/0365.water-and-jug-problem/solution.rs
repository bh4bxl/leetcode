// Created by bh4bxl at 2026/01/06 13:13
// leetgo: 1.4.16
// https://leetcode.com/problems/water-and-jug-problem/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a.abs() } else { gcd(b, a % b) }
        }

        if x + y < target {
            return false;
        }

        target % gcd(x, y) == 0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let x: i32 = deserialize(&read_line()?)?;
    let y: i32 = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_measure_water(x, y, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

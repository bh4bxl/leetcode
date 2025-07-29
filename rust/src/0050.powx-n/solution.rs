// Created by bh4bxl at 2025/07/28 22:03
// leetgo: 1.4.15
// https://leetcode.com/problems/powx-n/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let half = Self::my_pow(x, n / 2);

        if n % 2 == 0 {
            half * half
        } else if n > 0 {
            x * half * half
        } else {
            (1.0 / x) * half * half
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
	let x: f64 = deserialize(&read_line()?)?;
	let n: i32 = deserialize(&read_line()?)?;
	let ans: f64 = Solution::my_pow(x, n).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

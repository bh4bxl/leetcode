// Created by bh4bxl at 2025/07/21 17:01
// leetgo: 1.4.15
// https://leetcode.com/problems/divide-two-integers/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == divisor {
            return 1;
        }

        let mut i = 0;
        let sign = (dividend > 0) == (divisor > 0);
        let mut div = dividend.abs() as u32;
        let divor = divisor.abs() as u32;

        while div >= divor {
            let mut q = 0;
            while div > (divor << (q + 1)) {
                q += 1;
            }

            i += 1 << q;
            div -=  divor << q
        }

        if i == 1 << 31 && sign {
            i = i32::MAX;
        }

        match sign {
            true => i,
            false => -i,
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
	let dividend: i32 = deserialize(&read_line()?)?;
	let divisor: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::divide(dividend, divisor).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

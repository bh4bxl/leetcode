// Created by bh4bxl at 2026/01/12 14:51
// leetgo: 1.4.16
// https://leetcode.com/problems/sum-of-two-integers/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);

        while b != 0 {
            let carry = (a & b) << 1;
            a ^= b;
            b = carry;
        }

        a
    }
}

// @lc code=end

fn main() -> Result<()> {
    let a: i32 = deserialize(&read_line()?)?;
    let b: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::get_sum(a, b).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

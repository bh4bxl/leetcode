// Created by bh4bxl at 2025/10/20 14:15
// leetgo: 1.4.15
// https://leetcode.com/problems/bitwise-and-of-numbers-range/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut i = 0;
        let (mut l_m, mut r_m) = (left, right);
        while l_m != r_m {
            l_m >>= 1;
            r_m >>= 1;
            i += 1;
        }

        l_m << i
    }
}

// @lc code=end

fn main() -> Result<()> {
    let left: i32 = deserialize(&read_line()?)?;
    let right: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::range_bitwise_and(left, right).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

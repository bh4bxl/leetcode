// Created by bh4bxl at 2025/11/17 20:02
// leetgo: 1.4.15
// https://leetcode.com/problems/single-number-iii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_all = 0;
        for n in &nums {
            xor_all ^= n;
        }

        let diff_bit = xor_all & -xor_all;

        let (mut a, mut b) = (0, 0);

        for &n in &nums {
            if n & diff_bit == 0 {
                a ^= n;
            } else {
                b ^= n;
            }
        }

        vec![a, b]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::single_number(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

// Created by bh4bxl at 2025/08/04 22:21
// leetgo: 1.4.15
// https://leetcode.com/problems/sqrtx/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let (mut left, mut right) = (1, x / 2);

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid > x / mid {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        right
    }
}

// @lc code=end

fn main() -> Result<()> {
    let x: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::my_sqrt(x).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

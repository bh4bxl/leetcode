// Created by bh4bxl at 2026/01/06 13:18
// leetgo: 1.4.16
// https://leetcode.com/problems/valid-perfect-square/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let n = num as i64;
        let (mut left, mut right) = (1i64, n);

        while left <= right {
            let mid = left + (right - left) / 2;
            let sq = mid * mid;

            if sq == n {
                return true;
            } else if sq < n {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_perfect_square(num).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

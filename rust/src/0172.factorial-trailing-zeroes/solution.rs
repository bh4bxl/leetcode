// Created by bh4bxl at 2025/10/14 12:27
// leetgo: 1.4.15
// https://leetcode.com/problems/factorial-trailing-zeroes/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut res = 0;
        let mut n_m = n;

        while n_m > 4 {
            res += n_m / 5;
            n_m /= 5;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::trailing_zeroes(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

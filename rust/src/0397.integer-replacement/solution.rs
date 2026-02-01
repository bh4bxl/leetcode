// Created by bh4bxl at 2026/01/28 17:49
// leetgo: 1.4.16
// https://leetcode.com/problems/integer-replacement/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as i64;
        let mut step = 0;

        while n != 1 {
            if n & 0b1 == 0 {
                n >>= 1;
            } else {
                if n == 3 || (n & 2) == 0 {
                    n -= 1;
                } else {
                    n += 1;
                }
            }
            step += 1;
        }

        step
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::integer_replacement(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

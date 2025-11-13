// Created by bh4bxl at 2025/11/10 21:01
// leetgo: 1.4.15
// https://leetcode.com/problems/power-of-two/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n < 0 {
            return false;
        }

        let mut tmp: u32 = 1 << 31;
        let mut cnt = 0;

        for _i in 0..32 {
            let cur = n & tmp as i32;
            if cur != 0 {
                cnt += 1;
            }

            tmp = tmp >> 1;
        }

        cnt == 1
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_power_of_two(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

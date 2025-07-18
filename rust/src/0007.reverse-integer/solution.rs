// Created by bh4bxl at 2025/07/12 21:08
// leetgo: 1.4.14
// https://leetcode.com/problems/reverse-integer/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ret: i64 = 0;
        let mut lx = x as i64;

        while lx != 0 {
            let pop = lx % 10;
            lx /= 10;
            ret = ret * 10 + pop;
        }

        if ret > i32::MAX as i64 || ret < i32::MIN as i64{
            return 0;
        }

        ret as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
	let x: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::reverse(x).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

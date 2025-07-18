// Created by bh4bxl at 2025/07/12 22:16
// leetgo: 1.4.14
// https://leetcode.com/problems/string-to-integer-atoi/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign = 1;
        let mut ans: i64 = 0;
        let mut has_sign = false;

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if (c == '-' || c == '+') && !has_sign {
                sign = if c == '-' { -1 } else { 1 };
                has_sign = true;
                continue;
            }

            if c == ' ' && !has_sign {
                continue;
            }

            if c >= '0' && c <= '9' {
                has_sign = true;
                let pop = c as i64 - '0' as i64;

                if ans * sign > i32::MAX as i64 / 10 || ans * sign == i32::MAX as i64 / 10 && pop * sign > 7 {
                    return 2147483647;
                }
                if ans * sign < i32::MIN as i64 / 10 || ans * sign == i32::MIN as i64 / 10 && pop * sign < -8 {
                    return -2147483648;
                }
                ans = ans * 10 + pop;
            } else {
                break;
            }
        }

        (ans * sign) as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::my_atoi(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

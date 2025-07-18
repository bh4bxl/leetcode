// Created by bh4bxl at 2025/07/13 12:51
// leetgo: 1.4.14
// https://leetcode.com/problems/palindrome-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut rev = 0;
        let mut x_w = x;

        while x_w != 0 {
            let pop = x_w % 10;
            x_w /= 10;
            if rev > i32::MAX / 10 || rev < i32::MIN / 10 {
                break;
            }
            rev = rev * 10 + pop;
        }

        x == rev
    }
}

// @lc code=end

fn main() -> Result<()> {
	let x: i32 = deserialize(&read_line()?)?;
	let ans: bool = Solution::is_palindrome(x).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

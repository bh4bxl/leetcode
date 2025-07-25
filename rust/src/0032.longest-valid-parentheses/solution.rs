// Created by bh4bxl at 2025/07/23 21:56
// leetgo: 1.4.15
// https://leetcode.com/problems/longest-valid-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let (mut max, mut idx) = (0, 0);
        let mut stack = vec![];

        stack.push(-1);

        for c in s.chars() {
            match c {
                '(' => stack.push(idx),
                _ => {
                    stack.pop();
                    if stack.is_empty() {
                        stack.push(idx);
                    } else {
                        max = max.max(idx - stack.last().unwrap());
                    }
                },
            }
            idx += 1;
        }

        max
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::longest_valid_parentheses(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

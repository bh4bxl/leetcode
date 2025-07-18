// Created by bh4bxl at 2025/07/17 17:33
// leetgo: 1.4.15
// https://leetcode.com/problems/valid-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '(' {
                        stack.pop();
                    } else {
                        return false;
                    }
                },
                ']' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '[' {
                        stack.pop();
                    } else {
                        return false;
                    }
                },
                '}' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '{' {
                        stack.pop();
                    } else {
                        return false;
                    }
                },
                _ => continue,
            }
        }

        stack.is_empty()
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: bool = Solution::is_valid(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

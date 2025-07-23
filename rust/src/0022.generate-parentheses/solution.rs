// Created by bh4bxl at 2025/07/18 13:45
// leetgo: 1.4.15
// https://leetcode.com/problems/generate-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {

        fn backtrack(
            cur: String,
            left: usize,
            right: usize,
            n: usize) -> Vec<String>{
            let mut res: Vec<String> = vec![];
            if cur.len() == n * 2 {
                res.push(cur);
                return res;
            }

            if left < n {
                res.append(
                    &mut backtrack(cur.clone() + "(", left + 1, right, n)
                );
            }

            if right < left {
                res.append(
                    &mut backtrack(cur.clone() + ")", left, right + 1, n)
                );
            }

            res
        }

        backtrack(String::from(""), 0, 0, n as usize)
    }
}

// @lc code=end

fn main() -> Result<()> {
	let n: i32 = deserialize(&read_line()?)?;
	let ans: Vec<String> = Solution::generate_parenthesis(n).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

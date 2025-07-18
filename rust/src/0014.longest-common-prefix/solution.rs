// Created by bh4bxl at 2025/07/14 22:57
// leetgo: 1.4.14
// https://leetcode.com/problems/longest-common-prefix/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        let mut prefix = strs[0].clone();

        for i in 1..strs.len() {
            let min_lin = std::cmp::min(prefix.len(), strs[i].len());

            let mut j = 0;
            while  j < min_lin {
                if prefix.chars().nth(j).unwrap() !=
                    strs[i].chars().nth(j).unwrap() {
                        break;
                    }
                j += 1;
            }

            let p = &prefix[0..j];
            prefix = p.to_string();
        }

        prefix
    }
}

// @lc code=end

fn main() -> Result<()> {
	let strs: Vec<String> = deserialize(&read_line()?)?;
	let ans: String = Solution::longest_common_prefix(strs).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

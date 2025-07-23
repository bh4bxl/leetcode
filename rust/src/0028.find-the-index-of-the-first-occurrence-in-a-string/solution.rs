// Created by bh4bxl at 2025/07/21 16:33
// leetgo: 1.4.15
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let len = needle.len();
        let mut i: usize = 0;

        while i + len <= haystack.len() {
            let mut j = 0;
            while  j < len {
                if haystack.chars().nth(i + j).unwrap() !=
                    needle.chars().nth(j).unwrap() {
                    break;
                }
                j += 1;
            }
            println!("{i}-{j}");

            if j == len {
                return i as i32;
            }

            i += 1;
        }

        return -1;
    }
}

// @lc code=end

fn main() -> Result<()> {
	let haystack: String = deserialize(&read_line()?)?;
	let needle: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::str_str(haystack, needle).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

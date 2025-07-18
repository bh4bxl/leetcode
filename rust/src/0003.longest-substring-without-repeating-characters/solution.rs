// Created by bh4bxl at 2025/07/10 21:41
// leetgo: 1.4.14
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len_no_rep = 0;
        let mut win_cur_l: usize = 0;
        let mut win_cur_r: usize = 0;

        if s.len() == 0 {
            return 0;
        }

        while win_cur_r < s.len() {
            let win = &s[win_cur_l..win_cur_r];
            if win.find(s.chars().nth(win_cur_r).unwrap()).is_some() {
                win_cur_l += 1;
            } else {
                win_cur_r += 1;
                if max_len_no_rep < win.len() {
                    max_len_no_rep = win.len();
                }
            } 
        }

        max_len_no_rep += 1;

        max_len_no_rep as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::length_of_longest_substring(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

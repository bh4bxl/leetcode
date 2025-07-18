// Created by bh4bxl at 2025/07/11 15:49
// leetgo: 1.4.14
// https://leetcode.com/problems/longest-palindromic-substring/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let mut new_s = String::new();
        new_s.push('^');
        new_s.push('#');

        for c in s.chars() {
            new_s.push(c);
            new_s.push('#');
        }
        new_s.push('$');

        let mut dp: Vec<i32> = vec![0; new_s.len()];
        let mut max_right = 0;
        let mut center = 0;
        let mut max_len = 1;
        let mut begin = 0;

        let mut i = 0;
        while i < new_s.len() {
            if i < max_right {
                dp[i] = std::cmp::min(max_right - i, dp[2 * center - i] as usize) as i32;
            }

            let mut left = i as i32 - (1 + dp[i]);
            let mut right = i as i32 + (1 + dp[i]);
            while
                left >= 0 &&
                right < new_s.len() as i32 &&
                new_s.chars().nth(left as usize).unwrap() == new_s.chars().nth(right as usize).unwrap() {
                dp[i] += 1;
                left -= 1;
                right += 1;
            }

            if i + dp[i] as usize > max_right {
                max_right = i + dp[i] as usize;
                center = i;
            }

            if dp[i] as usize > max_len {
                max_len = dp[i] as usize;
                begin = (i - max_len) / 2;
            }
            i += 1;
        }

        s[begin..begin + max_len].to_string()
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: String = Solution::longest_palindrome(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

// Created by bh4bxl at 2025/07/25 14:36
// leetgo: 1.4.15
// https://leetcode.com/problems/count-and-say/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let last = Self::count_and_say(n - 1);

        return Self::next_str(&last);
    }

    fn next_str(s: &String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let cnt = Self::repeat_num(&s);
        let mut next = cnt.to_string();
        next.push(s.chars().nth(0).unwrap());
        let next_sub_str = &s[cnt..].to_string();
        next.push_str(Self::next_str(&next_sub_str).to_string().as_ref());
        return next;
    }

    fn repeat_num(s: &String) -> usize {
        let mut cnt = 0;
        let first = s.chars().nth(0).unwrap();

        for c in s.chars() {
            if c == first {
                cnt += 1;
            } else {
                break;
            }
        }

        cnt
    }
}

// @lc code=end

fn main() -> Result<()> {
	let n: i32 = deserialize(&read_line()?)?;
	let ans: String = Solution::count_and_say(n).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

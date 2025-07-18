// Created by bh4bxl at 2025/07/14 22:42
// leetgo: 1.4.14
// https://leetcode.com/problems/roman-to-integer/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;

        if s.contains("IV") { res -= 2; };
        if s.contains("IX") { res -= 2; };
        if s.contains("XL") { res -= 20; };
        if s.contains("XC") { res -= 20; };
        if s.contains("CD") { res -= 200; };
        if s.contains("CM") { res -= 200; };

        for count in 0..s.len() {
            let c = s.chars().nth(count).unwrap();
            match c {
                'M' => res += 1000,
                'D' => res += 500,
                'C' => res += 100,
                'L' => res += 50,
                'X' => res += 10,
                'V' => res += 5,
                'I' => res += 1,
                _ => res += 0,
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::roman_to_int(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

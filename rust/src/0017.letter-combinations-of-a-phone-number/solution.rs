// Created by bh4bxl at 2025/07/16 20:56
// leetgo: 1.4.15
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mapping = vec!["", "", "abc",
                           "def", "ghi", "jkl",
                           "mno", "pqrs", "tuv",
                           "wxyz"];
        let mut res: Vec<String> = Vec::new();
        if digits.len() == 0 {
            return res;
        }

        res.push(String::from(""));

        for i in 0..digits.len() {
            let x = digits
                .chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            while res[0].len() == i {
                let str = res.remove(0);
                for c in mapping[x].chars() {
                    let mut str_m = str.clone();
                    str_m.push(c);
                    res.push(str_m);
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let digits: String = deserialize(&read_line()?)?;
	let ans: Vec<String> = Solution::letter_combinations(digits).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

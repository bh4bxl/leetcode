// Created by bh4bxl at 2025/07/27 23:40
// leetgo: 1.4.15
// https://leetcode.com/problems/multiply-strings/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.eq("0") || num2.eq("0") {
            return String::from("0");
        }

        let mut res = String::new();
        let (l1, l2) = (num1.len(), num2.len());
        let mut pos = vec![0; l1 + l2];


        for i in (0..l1).rev() {
            for j in (0..l2).rev() {
                let n1 = num1.chars().nth(i).unwrap().to_digit(10).unwrap();
                let n2 = num2.chars().nth(j).unwrap().to_digit(10).unwrap();
                let sum = n1 * n2 + pos[i + j + 1];
                pos[i + j] += sum / 10;
                pos[i + j + 1] = sum % 10;
            }
        }

        let mut start = true;
        for i in pos {
            if start && i == 0 {
                start = false;
                continue;
            }
            res.push_str(&i.to_string());
            start = false;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let num1: String = deserialize(&read_line()?)?;
	let num2: String = deserialize(&read_line()?)?;
	let ans: String = Solution::multiply(num1, num2).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

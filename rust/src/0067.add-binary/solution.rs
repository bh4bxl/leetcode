// Created by bh4bxl at 2025/08/04 16:39
// leetgo: 1.4.15
// https://leetcode.com/problems/add-binary/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut l_chars: Vec<char> = a.chars().collect();
        let mut s_chars: Vec<char> = b.chars().collect();
        let mut res_chars: Vec<char> = vec![];
        let mut carry = 0;

        if a.len() < b.len() {
            l_chars = b.chars().collect();
            s_chars = a.chars().collect();
        }

        while !l_chars.is_empty() {
            let l = l_chars.pop().unwrap().to_digit(2).unwrap();
            let s = if s_chars.is_empty() {
                0
            } else {
                s_chars.pop().unwrap().to_digit(2).unwrap()
            };
            let sum = l + s + carry;
            carry = if sum > 1 { 1 } else { 0 };
            res_chars.push(if sum % 2 == 0 { '0' } else { '1' });
        }

        if carry == 1 {
            res_chars.push('1');
        }

        res_chars.reverse();
        res_chars.into_iter().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let a: String = deserialize(&read_line()?)?;
    let b: String = deserialize(&read_line()?)?;
    let ans: String = Solution::add_binary(a, b).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

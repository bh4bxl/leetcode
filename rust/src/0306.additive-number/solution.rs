// Created by bh4bxl at 2025/12/02 12:44
// leetgo: 1.4.15
// https://leetcode.com/problems/additive-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn dfs(a: i128, b: i128, idx: usize, s: &[u8]) -> bool {
            if idx == s.len() {
                return true;
            }

            let sum = a + b;
            let sum_str = sum.to_string();
            let len = sum_str.len();

            if idx + len > s.len() {
                return false;
            }

            if &s[idx..idx + len] != sum_str.as_bytes() {
                return false;
            }

            dfs(b, sum, idx + len, s)
        }

        fn to_num(s: &[u8]) -> i128 {
            let mut val = 0;
            for &c in s {
                val = val * 10 + (c - b'0') as i128;
            }
            val
        }

        let s = num.as_bytes();
        let n = s.len();

        for i in 1..n {
            if s[0] == b'0' && i > 1 {
                break;
            }

            let num1 = to_num(&s[0..i]);

            for j in i + 1..n {
                if s[i] == b'0' && (j - i) > 1 {
                    break;
                }

                let num2 = to_num(&s[i..j]);

                if dfs(num1, num2, j, s) {
                    return true;
                }
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_additive_number(num).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

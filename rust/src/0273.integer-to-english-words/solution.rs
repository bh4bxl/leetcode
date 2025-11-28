// Created by bh4bxl at 2025/11/24 20:19
// leetgo: 1.4.15
// https://leetcode.com/problems/integer-to-english-words/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let below_20 = [
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        let tens = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        let thousands = ["", "Thousand", "Million", "Billion"];

        fn helper(num: i32, below_20: &[&str], tens: &[&str]) -> String {
            if num == 0 {
                "".to_string()
            } else if num < 20 {
                below_20[num as usize].to_string()
            } else if num < 100 {
                let rest = helper(num % 10, below_20, tens);
                if rest.is_empty() {
                    tens[(num / 10) as usize].to_string()
                } else {
                    format!("{} {}", tens[(num / 10) as usize], rest)
                }
            } else {
                let rest = helper(num % 100, below_20, tens);
                if rest.is_empty() {
                    format!("{} Hundred", below_20[(num / 100) as usize])
                } else {
                    format!("{} Hundred {}", below_20[(num / 100) as usize], rest)
                }
            }
        }

        let mut num = num;
        let mut res = vec![];
        let mut i = 0;

        while num > 0 {
            let chunk = num % 1000;
            if chunk != 0 {
                let mut part = helper(chunk, &below_20, &tens);
                if !thousands[i].is_empty() {
                    part = format!("{} {}", part, thousands[i]);
                }
                res.push(part);
            }

            num /= 1000;
            i += 1;
        }

        res.reverse();
        res.join(" ")
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num: i32 = deserialize(&read_line()?)?;
    let ans: String = Solution::number_to_words(num).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

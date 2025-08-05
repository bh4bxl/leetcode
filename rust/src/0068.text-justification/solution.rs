// Created by bh4bxl at 2025/08/04 17:39
// leetgo: 1.4.15
// https://leetcode.com/problems/text-justification/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let mut i = 0;

        while i < words.len() {
            let mut line_len = 0;
            let mut words_len_sum = 0;
            let mut candidate = vec![];
            while i < words.len() && line_len + words[i].len() <= max_width as usize {
                words_len_sum += words[i].len();
                line_len += words[i].len();
                if line_len < max_width as usize {
                    line_len += 1;
                }
                candidate.push(&words[i]);
                i += 1;
            }
            let mut tag_str = String::from("");
            if candidate.len() == 1 {
                tag_str.push_str(&candidate[0]);
                let white_num = max_width as usize - candidate[0].len();
                for _ in 0..white_num {
                    tag_str.push(' ');
                }
            } else if i == words.len() {
                for j in 0..candidate.len() - 1 {
                    tag_str.push_str(&candidate[j]);
                    tag_str.push(' ');
                }
                tag_str.push_str(&candidate.last().unwrap());
                let white_num = max_width as usize - tag_str.len();
                for _ in 0..white_num {
                    tag_str.push(' ');
                }
            } else {
                let white_num = (max_width as usize - words_len_sum) / (candidate.len() - 1);
                let add_white = (max_width as usize - words_len_sum) % (candidate.len() - 1);
                for j in 0..candidate.len() - 1 {
                    tag_str.push_str(&candidate[j]);
                    for _ in 0..white_num {
                        tag_str.push(' ');
                    }
                    if j < add_white {
                        tag_str.push(' ');
                    }
                }
                tag_str.push_str(&candidate.last().unwrap());
            }

            res.push(tag_str);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let words: Vec<String> = deserialize(&read_line()?)?;
    let max_width: i32 = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::full_justify(words, max_width).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

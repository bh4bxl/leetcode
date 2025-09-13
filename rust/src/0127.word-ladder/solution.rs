// Created by bh4bxl at 2025/09/12 12:13
// leetgo: 1.4.15
// https://leetcode.com/problems/word-ladder/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: std::collections::HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut queue: std::collections::VecDeque<(String, i32)> =
            std::collections::VecDeque::new();
        queue.push_back((begin_word.clone(), 1));

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }

            let mut chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len() {
                let old = chars[i];
                for c in 'a'..='z' {
                    if c == old {
                        continue;
                    }

                    chars[i] = c;
                    let new_word: String = chars.iter().collect();
                    if word_set.contains(&new_word) {
                        queue.push_back((new_word.clone(), steps + 1));
                        word_set.remove(&new_word);
                    }
                }
                chars[i] = old;
            }
        }

        0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let begin_word: String = deserialize(&read_line()?)?;
    let end_word: String = deserialize(&read_line()?)?;
    let word_list: Vec<String> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::ladder_length(begin_word, end_word, word_list).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

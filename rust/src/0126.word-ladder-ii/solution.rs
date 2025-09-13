// Created by bh4bxl at 2025/09/11 16:31
// leetgo: 1.4.15
// https://leetcode.com/problems/word-ladder-ii/

use std::result;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_set: std::collections::HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return vec![];
        }

        let mut parents: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut level: std::collections::HashSet<String> = std::collections::HashSet::new();
        level.insert(begin_word.clone());
        let mut found = false;

        while !level.is_empty() && !found {
            for w in &level {
                word_set.remove(w);
            }

            let mut next_level: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            for word in &level {
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
                            if new_word == end_word {
                                found = true;
                            }

                            next_level.insert(new_word.clone());
                            parents
                                .entry(new_word.clone())
                                .or_insert(vec![])
                                .push(word.clone());
                        }
                    }
                    chars[i] = old;
                }
            }

            level = next_level;
        }

        let mut result = vec![];
        let mut path = vec![end_word.clone()];

        fn backtrack(
            word: &str,
            begin: &str,
            parents: &std::collections::HashMap<String, Vec<String>>,
            path: &mut Vec<String>,
            results: &mut Vec<Vec<String>>,
        ) {
            if word == begin {
                let mut seq = path.clone();
                seq.reverse();
                results.push(seq);
                return;
            }
            if let Some(prevs) = parents.get(word) {
                for p in prevs {
                    path.push(p.clone());
                    backtrack(p, begin, parents, path, results);
                    path.pop();
                }
            }
        }

        backtrack(&end_word, &begin_word, &parents, &mut path, &mut result);

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let begin_word: String = deserialize(&read_line()?)?;
    let end_word: String = deserialize(&read_line()?)?;
    let word_list: Vec<String> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<String>> = Solution::find_ladders(begin_word, end_word, word_list).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}

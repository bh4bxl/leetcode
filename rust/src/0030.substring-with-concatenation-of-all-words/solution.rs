// Created by bh4bxl at 2025/07/22 13:29
// leetgo: 1.4.15
// https://leetcode.com/problems/substring-with-concatenation-of-all-words/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = vec![];

        if words.is_empty() {
            return res;
        }

        let wd_num = words.len();
        let wd_len = words[0].len();
        let mut all_wd = std::collections::HashMap::new();

        for w in words {
            let mut val = 1;
            if let Some(v) = all_wd.get(&w) {
                val = v + 1
            }
            all_wd.insert(w, val);
        }

        for j in 0..wd_len {
            let mut has_wd = std::collections::HashMap::new();
            let mut num = 0;
            let mut i = j;

            while (i as i32) < s.len() as i32 - wd_len as i32 * wd_num as i32 + 1 {
                let mut has_removed = false;

                while num < wd_num {
                    let word = &s[i + wd_len * num..i + wd_len * (num + 1)];
                    if all_wd.contains_key(word) {
                        let mut val = 1;
                        if let Some(v) = has_wd.get(word) {
                            val = v + 1;
                        }
                        has_wd.insert(word.to_string(), val);

                        if has_wd.get(word).unwrap() > all_wd.get(word).unwrap() {
                            has_removed = true;
                            let mut remove_num = 0;

                            while has_wd.get(word).unwrap() > all_wd.get(word).unwrap() {
                                let first_word = &s[
                                    i + wd_len * remove_num..i + wd_len * (remove_num + 1)
                                ];
                                let v = has_wd.get(first_word).unwrap();
                                has_wd.insert(first_word.to_string(), v - 1);
                                remove_num += 1;
                            }

                            num = num - remove_num + 1;

                            i = i + wd_len * (remove_num - 1);
                            break;
                        }
                    } else {
                        has_wd.clear();
                        i = i + wd_len * num;
                        num = 0;
                        break;
                    }
                    num += 1;
                }

                if num == wd_num {
                    res.push(i as i32);
                }

                if num > 0 && !has_removed {
                    let first_word = &s[i..i + wd_len];
                    let val = has_wd.get(first_word).unwrap();
                    has_wd.insert(first_word.to_string(), val - 1);
                    num -= 1;
                }

                i += wd_len;
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let words: Vec<String> = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::find_substring(s, words).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

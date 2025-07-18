// Created by bh4bxl at 2025/07/14 15:35
// leetgo: 1.4.14
// https://leetcode.com/problems/integer-to-roman/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut res = String::new();
        let ones = ['I', 'X', 'C', 'M'];
        let fives = ['V', 'L', 'D'];
        let mut num_m = num;
        let mut count = 0;

        while num_m != 0 {
            let mut pop = num_m % 10;
            let mut str = String::new();

            if pop <= 3 {
                while pop != 0 {
                    str.push(ones[count]);
                    pop -= 1;
                }
            } else if pop == 4 {
                str.push(ones[count]);
                str.push(fives[count]);
            } else if pop == 5 {
                str.push(fives[count]);
            } else if pop > 5 && pop < 9 {
                str.push(fives[count]);
                pop -= 5;
                while pop != 0 {
                    str.push(ones[count]);
                    pop -= 1;
                }
            } else if pop == 9 {
                str.push(ones[count]);
                str.push(ones[count + 1]);
            }

            str.push_str(&res);

            res = str;

            count += 1;
            num_m /= 10;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let num: i32 = deserialize(&read_line()?)?;
	let ans: String = Solution::int_to_roman(num).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}

// Created by bh4bxl at 2025/11/03 12:36
// leetgo: 1.4.15
// https://leetcode.com/problems/implement-stack-using-queues/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct MyStack {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.data.pop().unwrap()
    }

    fn top(&self) -> i32 {
        self.data[self.data.len() - 1]
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = MyStack::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "push" => {
                let method_params = split_array(&params[i])?;
                let x: i32 = deserialize(&method_params[0])?;
                obj.push(x);
                output.push("null".to_string());
            }
            "pop" => {
                let ans: i32 = obj.pop().into();
                output.push(serialize(ans)?);
            }
            "top" => {
                let ans: i32 = obj.top().into();
                output.push(serialize(ans)?);
            }
            "empty" => {
                let ans: bool = obj.empty().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}

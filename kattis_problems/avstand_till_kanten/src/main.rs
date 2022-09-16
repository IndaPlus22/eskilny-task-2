/***
 * Built from Viola's template.
 */

use std::io;
use std::io::prelude::*;

fn main() {
    // taking input to set the program up
    let input = io::stdin();
    let mut lines = input.lock().lines(); // we've built an iterator over the lines input to stdin
    
    // Here we compute a vector representing all numbers on the first line
    let mut numbers: Vec<i32> = vec![];
    let next_line = lines.next()
        .expect("Invalid iostream.").expect("Error."); // expect error(I don't know why I need the second error handler!)
    let numbers_str = next_line 
        .split_whitespace(); // separate numbers into iterator, remove whitespaces: space, \n and \r
    for number_str in numbers_str {
        let j: i32 = number_str
            .trim() // remove whitespaces
            .parse() // parse as number
            .expect("Invalid input; second line must be a number."); // expect error
        numbers.push(j);
    }

    let r = numbers[0];
    let k = numbers[1];
    let mut r_i = r;
    while r_i != 0 {
        r_i -= 1;
        let mut k_i = k;
        while k_i != 0 {
            k_i -= 1;
            let val1 = if r-r_i < r_i+1 {r-r_i} else {r_i+1};
            let val2 = if k-k_i < k_i+1 {k-k_i} else {k_i+1};
            let print_val = if val1 < val2 {val1} else {val2};
            if print_val > 9 {
                print!(".");
            } else {
                print!("{}", print_val);
            }
        }
        println!();
    }
}
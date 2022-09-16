/***
 * Built from Viola's template.
 */

use std::io;
use std::io::prelude::*;

fn main() {
    // taking input to set the program up
    let input = io::stdin();
    let mut lines = input.lock().lines(); // we've built an iterator over the lines input to stdin
    
    loop {
        // Here we compute a vector representing all numbers on the first line
        let mut numbers: Vec<i128> = vec![];
        let next_line = lines.next()
            .expect("Invalid iostream.").expect("Error."); // expect error(I don't know why I need the second error handler!)
        let numbers_str = next_line 
            .split_whitespace(); // separate numbers into iterator, remove whitespaces: space, \n and \r
        for number_str in numbers_str {
            let j: i128 = number_str
                .trim() // remove whitespaces
                .parse() // parse as number
                .expect("Invalid input; second line must be a number."); // expect error
            numbers.push(j);
        }

        let i: i128 = numbers[0];
        let j: i128 = numbers[1];
        let diff = if i < j {j-i} else {i-j};
        println!{"{}", diff};
    }
}
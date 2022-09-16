/***
 * Built from Viola's template.
 */

use std::io;
use std::io::prelude::*;

fn main() {
    // taking input to set the program up
    let input = io::stdin();
    let mut lines = input.lock().lines(); // we've built an iterator over the lines input to stdin
    
    // number of inputs
    let count: f32 = lines.next() // we iterate over the first line
        .expect("Invalid iostream.").expect("Error.") // expect error (I don't know why I need the second error handler!)
        .trim() // remove whitespaces
        .parse() // parse as number
        .expect("Invalid input; first line must be a number."); // expect error

    // Here we compute a vector representing all numbers on line two
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
    
    // Next we sort the vector
    numbers.sort();

    // And sum all numbers which have indices according to the parameters in the question.
    let mut sum = 0;
    if count % 2.0 == 0.0 {
        for (i, x) in numbers.iter().enumerate() {
            if i as f32 >= count/2.0 {
                sum += x;
            }
        }
    } else {
        for (i, x) in numbers.iter().enumerate() {
            if i as f32 + 1.0 > count/2.0 {
                sum += x;
            }
        }
    }
    

    println!("{}", sum);
}
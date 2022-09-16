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
    let count: usize = lines.next() // we iterate over the first line
    .expect("Invalid iostream.").expect("Error.") // expect error (I don't know why I need the second error handler!)
    .trim() // remove whitespaces
    .parse() // parse as number
    .expect("Invalid input; first line must be a number."); // expect error

    // Here we compute a vector representing all numbers on the first line
    let mut names: Vec<String> = vec![];
    let mut i: usize = count;
    while i != 0 {
        i -= 1;
        let first_name = lines.next()
            .expect("Invalid iostream.").expect("Error."); // expect error(I don't know why I need the second error handler!)
        names.push(first_name);
    }
    
    while i != count {
        let last_name: &str = &lines.next()
            .expect("Invalid iostream.").expect("Error."); // expect error(I don't know why I need the second error handler!)
        names[i].push_str(" ");
        names[i].push_str(last_name);
        i += 1;
    }

    names.sort();
    names.dedup();
    println!("{}", names.len());
}
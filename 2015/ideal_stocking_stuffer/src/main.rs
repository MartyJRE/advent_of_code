use std::fmt::Write as FmtWrite;
use std::fs::read_to_string;

use md5;

/*
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
*/

fn part_one(input: &String) -> i32 {
    let mut idx = 0;
    let mut string = String::new();
    loop {
        let digest = md5::compute(format!("{input}{idx}"));
        for b in digest.0 {
            write!(string, "{:02x}", b).unwrap();
        }
        if string.starts_with("00000") {
            println!("Digits: {idx} = {string}");
            break;
        }
        idx += 1;
        string.clear();
    }
    idx
}

/*
--- Part Two ---

Now find one that starts with six zeroes.
*/

fn part_two(input: &String, idx_in: i32) {
    let mut idx = idx_in;
    let mut string = String::new();
    loop {
        let digest = md5::compute(format!("{input}{idx}"));
        for b in digest.0 {
            write!(string, "{:02x}", b).unwrap();
        }
        if string.starts_with("000000") {
            println!("Digits: {idx} = {string}");
            break;
        }
        idx += 1;
        string.clear();
    }
}

fn main() {
    let input_result = read_to_string("./input.txt");
    match input_result {
        Ok(input) => {
            let idx = part_one(&input);
            part_two(&input, idx);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

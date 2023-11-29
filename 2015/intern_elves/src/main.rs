use std::fs::read_to_string;

static DEBUG: bool = true;

/*
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:

    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    aaa is nice because it has at least three vowels and a double letter, even though the LETTERS used by different rules overlap.
    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?
*/

fn part_one(input: &String) {
    let mut nice = 0;
    for line in input.lines() {
        // check whether the line has 3 or more vowels
        let mut vowel_count = 0;
        for char in line.chars() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&char) {
                vowel_count += 1;
            }
        }
        if vowel_count < 3 {
            if DEBUG {
                println!("Miss: vowel_count={vowel_count} ({line})");
            }
            continue;
        }

        // check whether the line has a double letter
        let mut has_double_letter = false;
        let mut characters = line.chars().peekable();
        while let Some(character) = characters.next() {
            if let Some(next_character) = characters.peek() {
                if character == *next_character {
                    has_double_letter = true;
                    if DEBUG {
                        println!(
                            "Double letter: {has_double_letter} = {character} == {}",
                            *next_character
                        );
                    }
                    break;
                }
            }
        }
        if !has_double_letter {
            if DEBUG {
                println!("Miss: double_letter={has_double_letter} ({line})");
            }
            continue;
        }

        // check whether the line contains forbidden
        let mut contains_forbidden = false;
        for forbidden in ["ab", "cd", "pq", "xy"] {
            if line.contains(forbidden) {
                if DEBUG {
                    println!("Forbidden: {forbidden} in {line}");
                }
                contains_forbidden = true;
                break;
            }
        }
        if contains_forbidden {
            if DEBUG {
                println!("Miss: contains_forbidden={contains_forbidden} ({line})");
            }
            continue;
        }
        nice += 1;
    }
    println!("Nice count: {nice}");
}

/*
--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

    It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:

    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?
*/

fn part_two(input: &String) {
    let mut nice = 0;
    for line in input.lines() {
        // check for non overlapping pairs
        let mut has_non_overlapping_pair = false;
        let mut characters1 = line.chars().peekable();
        let mut pairs = Vec::<(String, i32)>::with_capacity(line.len());
        let mut idx = 0;
        while let Some(character) = characters1.next() {
            if let Some(other) = characters1.peek() {
                pairs.push((format!("{character}{other}"), idx));
            }
            idx += 1;
        }
        let mut pairs_iter = pairs.iter();
        while let Some((pair, index)) = pairs_iter.next() {
            if let Some((other_pair, other_index)) = pairs_iter.find(|(other_pair, other_index)| {
                *pair == *other_pair && (*index - *other_index).abs() > 1
            }) {
                has_non_overlapping_pair = true;
                if DEBUG {
                    println!(
                        "Non overlapping: {} {}, {} {} {line}",
                        *pair, *other_pair, *index, *other_index
                    );
                }
                break;
            }
        }
        if !has_non_overlapping_pair {
            if DEBUG {
                println!("Miss: has_non_overlapping_pair={has_non_overlapping_pair} ({line})");
            }
            continue;
        }

        // check for same characters separated by one
        let mut has_same_sep_by_one = false;
        let characters = line.chars().collect::<Vec<char>>();
        if characters.len() > 2 {
            for index in 0..characters.len() - 2 {
                let current = characters[index];
                let mid = characters[index + 1];
                let other = characters[index + 2];
                if current == other {
                    has_same_sep_by_one = true;
                    if DEBUG {
                        println!("Has two split by one: {current} {mid} {other} {line}");
                    }
                    break;
                }
            }
        }
        if !has_same_sep_by_one {
            if DEBUG {
                println!("Miss: has_same_sep_by_one={has_same_sep_by_one} ({line})");
            }
            continue;
        }

        nice += 1;
    }
    println!("Nice count: {nice}");
}

fn main() {
    let input_result = read_to_string("./input.txt");
    match input_result {
        Ok(input) => {
            part_one(&input);
            part_two(&input);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

use std::collections::HashMap;
use std::fs::read_to_string;

/*
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

    > delivers presents to 2 houses: one at the starting location, and one to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
*/

fn part_one(input: &String) {
    let mut map = HashMap::<(i32, i32), i32>::new();
    let (mut x, mut y) = (0, 0);
    map.insert((x, y), 1);
    for character in input.chars() {
        match character {
            '^' => {
                y += 1;
            }
            'v' => {
                y -= 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            _ => {}
        }
        match map.get(&(x, y)) {
            None => {
                map.insert((x, y), 1);
            }
            Some(item) => {
                map.insert((x, y), item + 1);
            }
        }
    }
    let count = map.iter().count();
    println!("Count: {}", count);
}

/*
--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
*/

fn part_two(input: &String) {
    let mut map = HashMap::<(i32, i32), i32>::new();
    let chars = input.chars().enumerate();
    let even = chars.clone().filter_map(|(i, character)| match i % 2 {
        0 => Some(character),
        _ => None,
    });
    let odd = chars.clone().filter_map(|(i, character)| match i % 2 {
        1 => Some(character),
        _ => None,
    });
    let (mut x, mut y) = (0, 0);
    for character in even {
        match character {
            '^' => {
                y += 1;
            }
            'v' => {
                y -= 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            _ => {}
        }
        match map.get(&(x, y)) {
            None => {
                map.insert((x, y), 1);
            }
            Some(item) => {
                map.insert((x, y), item + 1);
            }
        }
    }
    let (mut x, mut y) = (0, 0);
    for character in odd {
        match character {
            '^' => {
                y += 1;
            }
            'v' => {
                y -= 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            _ => {}
        }
        match map.get(&(x, y)) {
            None => {
                map.insert((x, y), 1);
            }
            Some(item) => {
                map.insert((x, y), item + 1);
            }
        }
    }
    let count = map.iter().count();
    println!("Count: {}", count);
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

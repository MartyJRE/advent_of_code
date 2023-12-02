use std::cmp::{max, min};
use std::fs::read_to_string;

/*
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:

    turn on 0,0 through 999,999 would turn on (or leave on) every light.
    toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?
*/
fn part_one(input: &String) {
    let mut lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    for line in input.lines() {
        if line.contains("turn on") {
            let shorter = line.replace("turn on ", "");
            let mut split = shorter.split(" ");
            let coords1_option = split.nth(0);
            let coords2 = split.map(|item| { item.replace("through", "") }).collect::<String>();
            match coords1_option {
                Some(coords1) => {
                    let split_1 = coords1.split(",").collect::<Vec<&str>>();
                    let split_2 = coords2.split(",").collect::<Vec<&str>>();
                    if split_1.len() != 2 || split_2.len() != 2 {
                        println!("Malformed line {line}");
                        continue;
                    }
                    let (x1, y1) = (split_1[0].parse::<usize>().unwrap(), split_1[1].parse::<usize>().unwrap());
                    let (x2, y2) = (split_2[0].parse::<usize>().unwrap(), split_2[1].parse::<usize>().unwrap());
                    let min_x = min(x1, x2);
                    let max_x = max(x1, x2);
                    let min_y = min(y1, y2);
                    let max_y = max(y1, y2);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            lights[x][y] = true;
                        }
                    }
                }
                _ => {}
            }
        } else if line.contains("turn off") {
            let shorter = line.replace("turn off ", "");
            let mut split = shorter.split(" ");
            let coords1_option = split.nth(0);
            let coords2 = split.map(|item| { item.replace("through", "") }).collect::<String>();
            match coords1_option {
                Some(coords1) => {
                    let split_1 = coords1.split(",").collect::<Vec<&str>>();
                    let split_2 = coords2.split(",").collect::<Vec<&str>>();
                    if split_1.len() != 2 || split_2.len() != 2 {
                        println!("Malformed line {line}");
                        continue;
                    }
                    let (x1, y1) = (split_1[0].parse::<usize>().unwrap(), split_1[1].parse::<usize>().unwrap());
                    let (x2, y2) = (split_2[0].parse::<usize>().unwrap(), split_2[1].parse::<usize>().unwrap());
                    let min_x = min(x1, x2);
                    let max_x = max(x1, x2);
                    let min_y = min(y1, y2);
                    let max_y = max(y1, y2);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            lights[x][y] = false;
                        }
                    }
                }
                _ => {}
            }
        } else if line.contains("toggle") {
            let shorter = line.replace("toggle ", "");
            let mut split = shorter.split(" ");
            let coords1_option = split.nth(0);
            let coords2 = split.map(|item| { item.replace("through", "") }).collect::<String>();
            match coords1_option {
                Some(coords1) => {
                    let split_1 = coords1.split(",").collect::<Vec<&str>>();
                    let split_2 = coords2.split(",").collect::<Vec<&str>>();
                    if split_1.len() != 2 || split_2.len() != 2 {
                        println!("Malformed line {line}");
                        continue;
                    }
                    let (x1, y1) = (split_1[0].parse::<usize>().unwrap(), split_1[1].parse::<usize>().unwrap());
                    let (x2, y2) = (split_2[0].parse::<usize>().unwrap(), split_2[1].parse::<usize>().unwrap());
                    let min_x = min(x1, x2);
                    let max_x = max(x1, x2);
                    let min_y = min(y1, y2);
                    let max_y = max(y1, y2);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            lights[x][y] = !lights[x][y];
                        }
                    }
                }
                _ => {}
            }
        } else {
            println!("Unknown line {line}")
        }
    }
    let mut on = 0;
    for light_row in lights.iter() {
        for &light in light_row.iter() {
            if light {
                on += 1;
            }
        }
    }
    println!("Lights on: {on}");
}

fn part_two(input: &String) {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    for line in input.lines() {
        if line.contains("turn on") {
            let shorter = line.replace("turn on ", "");
            let mut split = shorter.split(" ");
            let coords1_option = split.nth(0);
            let coords2 = split.map(|item| { item.replace("through", "") }).collect::<String>();
            match coords1_option {
                Some(coords1) => {
                    let split_1 = coords1.split(",").collect::<Vec<&str>>();
                    let split_2 = coords2.split(",").collect::<Vec<&str>>();
                    if split_1.len() != 2 || split_2.len() != 2 {
                        println!("Malformed line {line}");
                        continue;
                    }
                    let (x1, y1) = (split_1[0].parse::<usize>().unwrap(), split_1[1].parse::<usize>().unwrap());
                    let (x2, y2) = (split_2[0].parse::<usize>().unwrap(), split_2[1].parse::<usize>().unwrap());
                    let min_x = min(x1, x2);
                    let max_x = max(x1, x2);
                    let min_y = min(y1, y2);
                    let max_y = max(y1, y2);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            lights[x][y] = lights[x][y] + 1;
                        }
                    }
                }
                _ => {}
            }
        } else if line.contains("turn off") {
            let shorter = line.replace("turn off ", "");
            let mut split = shorter.split(" ");
            let coords1_option = split.nth(0);
            let coords2 = split.map(|item| { item.replace("through", "") }).collect::<String>();
            match coords1_option {
                Some(coords1) => {
                    let split_1 = coords1.split(",").collect::<Vec<&str>>();
                    let split_2 = coords2.split(",").collect::<Vec<&str>>();
                    if split_1.len() != 2 || split_2.len() != 2 {
                        println!("Malformed line {line}");
                        continue;
                    }
                    let (x1, y1) = (split_1[0].parse::<usize>().unwrap(), split_1[1].parse::<usize>().unwrap());
                    let (x2, y2) = (split_2[0].parse::<usize>().unwrap(), split_2[1].parse::<usize>().unwrap());
                    let min_x = min(x1, x2);
                    let max_x = max(x1, x2);
                    let min_y = min(y1, y2);
                    let max_y = max(y1, y2);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            if lights[x][y] != 0 {
                                lights[x][y] = lights[x][y] - 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        } else if line.contains("toggle") {
            let shorter = line.replace("toggle ", "");
            let mut split = shorter.split(" ");
            let coords1_option = split.nth(0);
            let coords2 = split.map(|item| { item.replace("through", "") }).collect::<String>();
            match coords1_option {
                Some(coords1) => {
                    let split_1 = coords1.split(",").collect::<Vec<&str>>();
                    let split_2 = coords2.split(",").collect::<Vec<&str>>();
                    if split_1.len() != 2 || split_2.len() != 2 {
                        println!("Malformed line {line}");
                        continue;
                    }
                    let (x1, y1) = (split_1[0].parse::<usize>().unwrap(), split_1[1].parse::<usize>().unwrap());
                    let (x2, y2) = (split_2[0].parse::<usize>().unwrap(), split_2[1].parse::<usize>().unwrap());
                    let min_x = min(x1, x2);
                    let max_x = max(x1, x2);
                    let min_y = min(y1, y2);
                    let max_y = max(y1, y2);
                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            lights[x][y] = lights[x][y] + 2;
                        }
                    }
                }
                _ => {}
            }
        } else {
            println!("Unknown line {line}")
        }
    }
    let mut brightness: u32 = 0;
    for light_row in lights.iter() {
        for &light in light_row.iter() {
            brightness += u32::from(light);
        }
    }
    println!("Total brightness: {brightness}");
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


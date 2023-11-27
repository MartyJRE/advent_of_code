use std::cmp::min;
use std::fs::read_to_string;

/*
The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
*/

fn part_one(input: &String) {
    let mut total = 0;
    for line in input.lines() {
        let dimensions = line.split("x").collect::<Vec<&str>>();
        match dimensions[..] {
            [x_str, y_str, z_str] => {
                let x = x_str.parse::<i32>().unwrap();
                let y = y_str.parse::<i32>().unwrap();
                let z = z_str.parse::<i32>().unwrap();
                let xy = x * y;
                let xz = x * z;
                let yz = y * z;

                total += (xy * 2 + xz * 2 + yz * 2);
                total += min(xy, min(xz, yz));
            }
            _ => {}
        }
    }
    println!("Wrapping: {}", total);
}

/*
--- Part Two ---

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.

For example:

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?
*/

fn part_two(input: &String) {
    let mut total = 0;
    for line in input.lines() {
        let dimensions = line.split("x").collect::<Vec<&str>>();
        match dimensions[..] {
            [x_str, y_str, z_str] => {
                let x = x_str.parse::<i32>().unwrap();
                let y = y_str.parse::<i32>().unwrap();
                let z = z_str.parse::<i32>().unwrap();

                if y <= x && z <= x {
                    // println!(" {:0>2}  ({:1>2}) ({:2>2})", x, y, z);
                    total += y + y + z + z;
                } else if x <= y && z <= y {
                    // println!("({:0>2})  {:1>2}  ({:2>2})", x, y, z);
                    total += x + x + z + z;
                } else {
                    // println!("({:0>2}) ({:1>2})  {:2>2}", x, y, z);
                    total += x + x + y + y;
                }
                total += x * y * z;
            }
            _ => {}
        }
    }
    println!("Ribbon: {}", total);
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

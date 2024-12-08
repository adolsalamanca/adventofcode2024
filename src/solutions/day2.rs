enum Op {
    Add,
    Sub,
    Unknown,
}

pub(crate) fn main(){
    let data: &str = include_str!("../input/day2.txt");
    println!("day 2: find safe levels.");
    println!("safe levels part1: {}",calculate_safe_lines(data));
    println!("safe levels part2: {}", calculate_safe_lines_with_one_exception(data));

}

fn calculate_safe_lines(input_str: &str) -> u32{
    let mut safe_lines:u32 = 0;

    input_str.lines().for_each(|l| {
        let line_numbers = l.split(' ').map(|c| {
            return c.parse::<u32>().expect("could not parse numbers");
        }).collect::<Vec<u32>>();

        let mut safe:bool = true;
        let mut last:u32 = u32::MAX;
        let mut current_op : Op = Op::Unknown;

        for i in 0..line_numbers.len(){
            if last == u32::MAX {
                last = line_numbers[i];
                continue;
            }
            let current_number = line_numbers[i];

                match current_op {
                    Op::Add => {
                        if is_greater_safe(current_number, last) {
                            last = current_number;
                            continue;
                        } else {
                            safe = false;
                            break;
                        }
                    }
                    Op::Sub => {
                        if is_lower_safe(current_number, last) {
                            last = current_number;
                            continue;
                        } else {
                            safe = false;
                            break;
                        }
                    }

                    Op::Unknown => {
                        if is_greater_safe(current_number, last) {
                            current_op = Op::Add;
                            last = current_number;
                        } else if is_lower_safe(current_number, last) {
                            current_op = Op::Sub;
                            last = current_number;
                        } else {
                            safe = false;
                            break;
                        }
                    }
                }
        }

        if safe {
            safe_lines+=1;
        }
    });

    safe_lines
}

fn calculate_safe_lines_with_one_exception(input_str: &str) -> u32{
    let mut safe_lines:u32 = 0;

    input_str.lines().for_each(|l| {
        let line_numbers:Vec<u32> = l.split(' ').map(|c| {
            return c.parse::<u32>().expect("could not parse numbers");
        }).collect::<Vec<u32>>();

        if is_one_line_variation_safe(line_numbers) {
            safe_lines+=1;
        }
    });

    safe_lines
}

fn is_one_line_variation_safe(l: Vec<u32>) -> bool {
    for i in 0..l.len(){
        let mut cloned_line = l.clone();
        cloned_line.remove(i);

        if calculate_safe_lines(&*cloned_line.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(" ")) > 0 {
            return true;
        };
    }

    false
}

fn is_greater_safe(n1: u32, n2: u32) -> bool {
    n1 > n2 && (n1 - n2) <= 3
}

fn is_lower_safe(n1: u32, n2: u32) -> bool {
    n2 > n1 && (n2 - n1) <= 3
}

#[test]
fn should_identify_line_as_safe(){
    let input_str: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    let safe_lines = calculate_safe_lines(input_str);
    assert_eq!(2, safe_lines);
}

#[test]
fn should_identify_line_as_safe_removing_one_level(){
    let input_str: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    let safe_lines = calculate_safe_lines_with_one_exception(input_str);
    assert_eq!(4, safe_lines);
}



/*
--- Day 2: Red-Nosed Reports ---

Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?
*/
use std::str::Split;

fn main() {
    let input_raw = include_str!("input.txt");
    let mut parts = input_raw.trim().split('\n');

    println!("Part 1: {}", part_1(&mut parts.clone()));
    println!("Part 2: {}", part_2(&mut parts));
}

fn part_1(input: &mut Split<char>) -> i32 {
    let (mut x, mut y) = (0, 0);

    for command in input {
        let parts: Vec<&str> = command.split(' ').collect();
        let count: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => {
                x += count;
            }
            "up" => {
                y -= count;
            }
            "down" => {
                y += count;
            }
            _ => panic!(),
        }
    }

    x * y
}

fn part_2(input: &mut Split<char>) -> i32 {
    let (mut x, mut y, mut aim) = (0, 0, 0);

    for command in input {
        let parts: Vec<&str> = command.split(' ').collect();
        let count: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => {
                x += count;
                y += count * aim;
            }
            "up" => {
                aim -= count;
            }
            "down" => {
                aim += count;
            }
            _ => panic!(),
        }
    }

    x * y
}

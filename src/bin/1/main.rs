use std::{fs::File, io::Read};

fn main() {
    let mut input_file = File::open("./src/bin/1/input.txt").unwrap();
    let mut input_raw = String::new();
    input_file.read_to_string(&mut input_raw).unwrap();

    let input: Vec<usize> = input_raw
        .trim()
        .split('\n')
        .map(|str| str.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[usize]) -> usize {
    let mut increases: usize = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increases += 1;
        }
    }

    increases
}

fn part_2(input: &[usize]) -> usize {
    let mut increases: usize = 0;

    for i in 1..(input.len() - 2) {
        if (input[i..=(i + 2)]).iter().sum::<usize>()
            > (input[(i - 1)..=(i + 1)]).iter().sum::<usize>()
        {
            increases += 1;
        }
    }

    increases
}

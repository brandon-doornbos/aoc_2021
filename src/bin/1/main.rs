fn main() {
    let input_raw = include_str!("input.txt");
    let input: Vec<i32> = input_raw
        .trim()
        .split('\n')
        .map(|str| str.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[i32]) -> i32 {
    let mut increases = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increases += 1;
        }
    }

    increases
}

fn part_2(input: &[i32]) -> i32 {
    let mut increases = 0;

    for i in 1..(input.len() - 2) {
        if (input[i..=(i + 2)]).iter().sum::<i32>() > (input[(i - 1)..=(i + 1)]).iter().sum::<i32>()
        {
            increases += 1;
        }
    }

    increases
}

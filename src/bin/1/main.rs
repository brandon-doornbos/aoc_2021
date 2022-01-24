fn main() {
    let input_raw = include_str!("input.txt");
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

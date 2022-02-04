const BIRTH_AGE: usize = 0;
const RESET_AGE: usize = 6;
const INITIAL_AGE: usize = 8;

pub fn main() {
    let input_raw = include_str!("input.txt");
    let input: Vec<usize> = input_raw
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1_2(&input, 80));
    println!("Part 2: {}", part_1_2(&input, 256));
}

fn part_1_2(input: &[usize], days: u32) -> u64 {
    let mut ages: [u64; INITIAL_AGE + 1] = [0; INITIAL_AGE + 1];

    for age in input {
        ages[*age] += 1;
    }

    for _ in 0..days {
        let mut reset_fish: u64 = 0;
        let mut new_fish: u64 = 0;
        for i in BIRTH_AGE..=INITIAL_AGE {
            if i == BIRTH_AGE {
                new_fish = ages[BIRTH_AGE];
                reset_fish = ages[BIRTH_AGE];
                ages[BIRTH_AGE] = 0;
            } else {
                ages[i - 1] += ages[i];
                ages[i] = 0;
            }
        }
        ages[INITIAL_AGE] += new_fish;
        ages[RESET_AGE] += reset_fish;
    }

    let mut fish_amt = 0;
    for fish in ages {
        fish_amt += fish;
    }
    fish_amt
}

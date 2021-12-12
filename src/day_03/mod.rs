use std::{fs::File, io::Read};

const BIT_LENGTH: usize = 12;

pub fn main() {
    let mut input_file = File::open("./src/day_03/input.txt").unwrap();
    let mut input_raw = String::new();
    input_file.read_to_string(&mut input_raw).unwrap();

    let input: Vec<&str> = input_raw.trim().split('\n').collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[&str]) -> usize {
    let mut bits: [[usize; BIT_LENGTH]; 2] = [[0; BIT_LENGTH]; 2];

    for line in input {
        let chars = line.chars().enumerate();
        for (i, char) in chars {
            if char == '1' {
                bits[1][i] += 1;
            } else {
                bits[0][i] += 1;
            }
        }
    }

    let mut epsilon = String::with_capacity(5);
    let mut gamma = String::with_capacity(5);
    for i in 0..BIT_LENGTH {
        if bits[0][i] > bits[1][i] {
            epsilon += "1";
            gamma += "0";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    binary_to_decimal(epsilon) * binary_to_decimal(gamma)
}

fn part_2(input: &[&str]) -> usize {
    fn process_input(input: &[&str], greater: bool) -> usize {
        let mut result: Vec<String> = input.iter().map(|str| String::from(*str)).collect();
        let mut index = 0;

        while result.len() > 1 {
            let mut bits: [[usize; BIT_LENGTH]; 2] = [[0; BIT_LENGTH]; 2];

            for line in &result {
                let chars = line.chars().enumerate();
                for (i, char) in chars {
                    if char == '1' {
                        bits[1][i] += 1;
                    } else {
                        bits[0][i] += 1;
                    }
                }
            }

            let comp: char = if bits[0][index] > bits[1][index] {
                if greater {
                    '0'
                } else {
                    '1'
                }
            } else if bits[0][index] < bits[1][index] {
                if greater {
                    '1'
                } else {
                    '0'
                }
            } else {
                '&'
            };

            let mut new_result: Vec<String> = vec![];

            for line in &result {
                let chars = line.chars().collect::<Vec<char>>();
                if (comp == '&' && chars[index] != (if greater { '0' } else { '1' }))
                    || comp == chars[index]
                {
                    new_result.push(String::from(line.clone()));
                }
            }

            result = new_result;
            index += 1;
        }

        binary_to_decimal(result[0].clone())
    }

    process_input(input.clone(), true) * process_input(input.clone(), false)
}

fn binary_to_decimal(binary: String) -> usize {
    let mut result: usize = 0;

    for (i, char) in binary.chars().enumerate() {
        if char == '1' {
            result += 2_usize.pow((binary.len() - i - 1).try_into().unwrap());
        }
    }

    result
}

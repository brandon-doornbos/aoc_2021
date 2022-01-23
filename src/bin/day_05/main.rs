use std::{fs::File, io::Read};

const FLOOR_SZ: usize = 1000;

pub fn main() {
    let mut input_file = File::open("./src/bin/day_05/input.txt").unwrap();
    let mut input_raw = String::new();
    input_file.read_to_string(&mut input_raw).unwrap();
    let input: Vec<usize> = input_raw
        .trim()
        .split(['\n', ' ', ','])
        .filter(|x| *x != "->")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Vec<usize>) -> usize {
    let mut floor: Vec<usize> = vec![0; FLOOR_SZ * FLOOR_SZ];

    let mut i: usize = 0;
    while i < input.len() {
        let x1: usize = input[i + 0];
        let y1: usize = input[i + 1];
        let x2: usize = input[i + 2];
        let y2: usize = input[i + 3];
        i += 4;

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                floor[y * FLOOR_SZ + x1] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                floor[y1 * FLOOR_SZ + x] += 1;
            }
        }
    }

    let mut overlap: usize = 0;
    for point in floor {
        if point > 1 {
            overlap += 1;
        }
    }

    overlap
}

fn part_2(input: &Vec<usize>) -> usize {
    let mut floor: Vec<usize> = vec![0; FLOOR_SZ * FLOOR_SZ];

    let mut i: usize = 0;
    while i < input.len() {
        let x1: usize = input[i + 0];
        let y1: usize = input[i + 1];
        let x2: usize = input[i + 2];
        let y2: usize = input[i + 3];
        i += 4;

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                floor[y * FLOOR_SZ + x1] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                floor[y1 * FLOOR_SZ + x] += 1;
            }
        } else {
            let mut x: usize = x1;
            let mut y: usize = y1;
            if x1 <= x2 {
                if y1 <= y2 {
                    while {
                        floor[y * FLOOR_SZ + x] += 1;
                        x += 1;
                        y += 1;

                        x <= x2 && y <= y2
                    } {}
                } else {
                    while {
                        floor[y * FLOOR_SZ + x] += 1;
                        x += 1;
                        y -= 1;

                        x <= x2 && y >= y2
                    } {}
                }
            } else {
                if y1 <= y2 {
                    while {
                        floor[y * FLOOR_SZ + x] += 1;
                        x -= 1;
                        y += 1;

                        x >= x2 && y <= y2
                    } {}
                } else {
                    while {
                        floor[y * FLOOR_SZ + x] += 1;
                        x -= 1;
                        y -= 1;

                        x >= x2 && y >= y2
                    } {}
                }
            }
        }
    }

    let mut overlap: usize = 0;
    for point in floor {
        if point > 1 {
            overlap += 1;
        }
    }

    overlap
}

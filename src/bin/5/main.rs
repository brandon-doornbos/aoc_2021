const FLOOR_SZ: usize = 1000;

pub fn main() {
    let input_raw = include_str!("input.txt");
    let input: Vec<usize> = input_raw
        .trim()
        .split(['\n', ' ', ','])
        .filter(|x| *x != "->")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[usize]) -> i32 {
    let mut floor: Vec<i32> = vec![0; FLOOR_SZ * FLOOR_SZ];

    let mut i = 0;
    while i < input.len() {
        let x1 = input[i];
        let y1 = input[i + 1];
        let x2 = input[i + 2];
        let y2 = input[i + 3];
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

    let mut overlap: i32 = 0;
    for point in floor {
        if point > 1 {
            overlap += 1;
        }
    }

    overlap
}

fn part_2(input: &[usize]) -> i32 {
    let mut floor: Vec<i32> = vec![0; FLOOR_SZ * FLOOR_SZ];

    let mut i = 0;
    while i < input.len() {
        let x1 = input[i];
        let y1 = input[i + 1];
        let x2 = input[i + 2];
        let y2 = input[i + 3];
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
            let mut x = x1;
            let mut y = y1;
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
            } else if y1 <= y2 {
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

    let mut overlap: i32 = 0;
    for point in floor {
        if point > 1 {
            overlap += 1;
        }
    }

    overlap
}

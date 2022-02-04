const STEPS: i32 = 100;
const GRID_SZ: usize = 10;
type Grid = [[i32; GRID_SZ]; GRID_SZ];

fn main() {
    let input_raw = include_str!("input.txt").trim();

    let mut input: Grid = Default::default();
    for (i, line) in input_raw.split('\n').enumerate() {
        for (j, energy) in line.split("").filter_map(|s| s.parse().ok()).enumerate() {
            input[i][j] = energy;
        }
    }

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Grid) -> i32 {
    let mut octopi: Grid = *input;
    let mut flashes = 0;

    for _ in 0..STEPS {
        for row in &mut octopi {
            for energy in row {
                *energy += 1;
            }
        }

        let mut found_flashes = true;
        while found_flashes {
            found_flashes = false;
            for i in 0..octopi.len() {
                for j in 0..octopi[i].len() {
                    if octopi[i][j] > 9 {
                        found_flashes = true;
                        flash(i, j, &mut octopi);
                        flashes += 1;
                    }
                }
            }
        }

        for row in &mut octopi {
            for energy in row {
                if *energy == -1 {
                    *energy = 0;
                }
            }
        }
    }

    flashes
}

fn part_2(input: &Grid) -> i32 {
    let mut octopi: Grid = *input;

    let mut step = 0;
    loop {
        let mut flashes = 0;

        for row in &mut octopi {
            for energy in row {
                *energy += 1;
            }
        }

        let mut found_flashes = true;
        while found_flashes {
            found_flashes = false;
            for i in 0..octopi.len() {
                for j in 0..octopi[i].len() {
                    if octopi[i][j] > 9 {
                        found_flashes = true;
                        flash(i, j, &mut octopi);
                        flashes += 1;
                    }
                }
            }
        }

        for row in &mut octopi {
            for energy in row {
                if *energy == -1 {
                    *energy = 0;
                }
            }
        }

        step += 1;
        if flashes == GRID_SZ * GRID_SZ {
            break step;
        }
    }
}

fn flash(i: usize, j: usize, octopi: &mut Grid) {
    octopi[i][j] = -1;

    let top = j != 0;
    let right = i != octopi.len() - 1;
    let bottom = j != octopi[i].len() - 1;
    let left = i != 0;

    if top && octopi[i][j - 1] != -1 {
        octopi[i][j - 1] += 1;
    }
    if top && right && octopi[i + 1][j - 1] != -1 {
        octopi[i + 1][j - 1] += 1;
    }
    if right && octopi[i + 1][j] != -1 {
        octopi[i + 1][j] += 1;
    }
    if bottom && right && octopi[i + 1][j + 1] != -1 {
        octopi[i + 1][j + 1] += 1;
    }
    if bottom && octopi[i][j + 1] != -1 {
        octopi[i][j + 1] += 1;
    }
    if bottom && left && octopi[i - 1][j + 1] != -1 {
        octopi[i - 1][j + 1] += 1;
    }
    if left && octopi[i - 1][j] != -1 {
        octopi[i - 1][j] += 1;
    }
    if top && left && octopi[i - 1][j - 1] != -1 {
        octopi[i - 1][j - 1] += 1;
    }
}

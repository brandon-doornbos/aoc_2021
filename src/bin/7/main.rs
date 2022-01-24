const MAX_POSITION: usize = 2000;

pub fn main() {
    let input_raw = include_str!("input.txt");
    let input: Vec<usize> = input_raw
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1_2(&input, &|dist| dist));
    println!(
        "Part 2: {}",
        part_1_2(&input, &|dist| (dist * (dist + 1)) / 2)
    );
}

fn part_1_2(input: &[usize], fuel_cost_func: &dyn Fn(usize) -> usize) -> usize {
    let mut crabs_pos: [usize; MAX_POSITION] = [0; MAX_POSITION];
    let mut fuel_costs: [usize; MAX_POSITION] = [0; MAX_POSITION];

    for pos in input {
        crabs_pos[*pos] += 1;
    }

    for i in 0..MAX_POSITION {
        for j in 0..MAX_POSITION {
            let dist = ((i as isize) - (j as isize)).abs() as usize;
            fuel_costs[i] += crabs_pos[j] * fuel_cost_func(dist);
        }
    }

    fuel_costs.sort_unstable();
    fuel_costs[0]
}

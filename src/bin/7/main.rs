const MAX_POSITION: usize = 2000;

fn main() {
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

fn part_1_2(input: &[usize], fuel_cost_func: &dyn Fn(i32) -> i32) -> i32 {
    let mut crabs_pos: [i32; MAX_POSITION] = [0; MAX_POSITION];
    let mut fuel_costs: [i32; MAX_POSITION] = [0; MAX_POSITION];

    for pos in input {
        crabs_pos[*pos] += 1;
    }

    for (i, fuel) in fuel_costs.iter_mut().enumerate() {
        for (j, crab) in crabs_pos.iter().enumerate() {
            let dist = ((i as i32) - (j as i32)).abs();
            *fuel += crab * fuel_cost_func(dist);
        }
    }

    fuel_costs.sort_unstable();
    fuel_costs[0]
}

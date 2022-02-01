pub fn main() {
    let input_raw = include_str!("input.txt").trim();
    let mut input = vec![];

    for row in input_raw.split('\n') {
        let nums: Vec<usize> = row.split("").filter_map(|s| s.parse().ok()).collect();
        input.push(nums);
    }

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[Vec<usize>]) -> usize {
    let mut risk_sum: usize = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            if (j != 0 && input[i][j - 1] <= *point)
                || (j != row.len() - 1 && input[i][j + 1] <= *point)
                || (i != 0 && input[i - 1][j] <= *point)
                || (i != input.len() - 1 && input[i + 1][j] <= *point)
            {
                continue;
            }

            risk_sum += point + 1;
        }
    }

    risk_sum
}

fn part_2(input: &[Vec<usize>]) -> usize {
    let mut low_points: Vec<(usize, usize, usize)> = vec![];
    let mut basins: Vec<usize> = vec![];

    let input_len = input.len();
    let row_len = input[0].len();

    for (i, row) in input.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            if (j != 0 && input[i][j - 1] <= *point)
                || (j != row_len - 1 && input[i][j + 1] <= *point)
                || (i != 0 && input[i - 1][j] <= *point)
                || (i != input_len - 1 && input[i + 1][j] <= *point)
            {
                continue;
            }

            low_points.push((i, j, basins.len()));
            basins.push(0);
        }
    }

    let mut visited = vec![vec![false; row_len]; input_len];
    let mut to_visit = low_points;

    while !to_visit.is_empty() {
        for i in (0..to_visit.len()).rev() {
            let (x, y, basin) = to_visit[i];
            if y != 0 && input[x][y - 1] != 9 && !visited[x][y - 1] {
                to_visit.push((x, y - 1, basin));
                basins[basin] += 1;
                visited[x][y - 1] = true;
            }
            if y != row_len - 1 && input[x][y + 1] != 9 && !visited[x][y + 1] {
                to_visit.push((x, y + 1, basin));
                basins[basin] += 1;
                visited[x][y + 1] = true;
            }
            if x != 0 && input[x - 1][y] != 9 && !visited[x - 1][y] {
                to_visit.push((x - 1, y, basin));
                basins[basin] += 1;
                visited[x - 1][y] = true;
            }
            if x != input_len - 1 && input[x + 1][y] != 9 && !visited[x + 1][y] {
                to_visit.push((x + 1, y, basin));
                basins[basin] += 1;
                visited[x + 1][y] = true;
            }
            to_visit.remove(i);
        }
    }

    basins.sort_unstable();
    let mut result = basins[basins.len() - 3];
    for basin in basins.iter().skip(basins.len() - 2) {
        result *= basin;
    }
    result
}

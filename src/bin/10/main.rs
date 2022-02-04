use std::collections::HashMap;

fn main() {
    let input_raw = include_str!("input.txt").trim();
    let input: Vec<&str> = input_raw.split('\n').collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[&str]) -> i32 {
    let point_lut = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut points = 0;

    let chunk_types = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for line in input {
        let mut chunks: Vec<char> = vec![];

        for (i, character) in line.chars().enumerate() {
            if chunk_types.contains_key(&character) {
                chunks.push(character);
            } else if chunk_types[chunks.last().unwrap()] == character {
                chunks.pop();
            } else if i != line.len() - 1 {
                points += point_lut[&character];
                break;
            }
        }
    }

    points
}

fn part_2(input: &[&str]) -> u64 {
    let point_lut = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores = vec![];

    let chunk_types = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for line in input {
        let mut chunks: Vec<char> = vec![];

        for (i, character) in line.chars().enumerate() {
            if chunk_types.contains_key(&character) {
                chunks.push(character);
            } else if chunk_types[chunks.last().unwrap()] == character {
                chunks.pop();
            } else if i != line.len() - 1 {
                break;
            }

            if i == line.len() - 1 {
                let mut score: u64 = 0;
                for bracket in chunks.iter().rev() {
                    score *= 5;
                    score += point_lut[&chunk_types[bracket]];
                }
                scores.push(score);
                break;
            }
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

const NUMBER_BASE: usize = 10;
const OUTPUT_VALUE_COUNT: usize = 4;
const DIGIT_SEGMENT_COUNT: [usize; NUMBER_BASE] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
const INPUT_LINE_LENGTH: usize = NUMBER_BASE + OUTPUT_VALUE_COUNT;
const SEGMENT_COUNT: usize = 7;

pub fn main() {
    let input_raw = include_str!("input.txt");
    let input: Vec<&str> = input_raw
        .trim()
        .split(['|', ' ', '\n'])
        .filter(|x| !x.is_empty())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[&str]) -> i32 {
    let mut unique_values = 0;

    for i in (0..input.len()).step_by(NUMBER_BASE + OUTPUT_VALUE_COUNT) {
        for j in 0..OUTPUT_VALUE_COUNT {
            let out_value_len = input[i + NUMBER_BASE + j].len();
            if out_value_len == DIGIT_SEGMENT_COUNT[1]
                || out_value_len == DIGIT_SEGMENT_COUNT[4]
                || out_value_len == DIGIT_SEGMENT_COUNT[7]
                || out_value_len == DIGIT_SEGMENT_COUNT[8]
            {
                unique_values += 1;
            }
        }
    }

    unique_values
}

fn part_2(input: &[&str]) -> i32 {
    let mut total = 0;

    for i in (0..input.len()).step_by(INPUT_LINE_LENGTH) {
        let mut segment_signal: [u8; SEGMENT_COUNT] = [0; SEGMENT_COUNT];
        // mapped top-to-bottom, left-to-right, e.g. the segments on the right are index 2 on the top and index 5 on the bottom.

        for j in 0..NUMBER_BASE {
            let signal = input[i + j];
            if signal.len() == DIGIT_SEGMENT_COUNT[1] {
                let signal_bitmask = segments_to_bitmask(signal, None);
                segment_signal[2] = signal_bitmask;
                segment_signal[5] = signal_bitmask;
                break;
            }
        }

        for j in 0..NUMBER_BASE {
            let signal = input[i + j];
            if signal.len() == DIGIT_SEGMENT_COUNT[7] {
                segment_signal[0] = segment_signal[2] ^ segments_to_bitmask(signal, None);
                break;
            }
        }

        for j in 0..NUMBER_BASE {
            let signal = input[i + j];
            if signal.len() == DIGIT_SEGMENT_COUNT[4] {
                let signal_bitmask = segments_to_bitmask(signal, None);
                segment_signal[1] = segment_signal[2] ^ signal_bitmask;
                segment_signal[3] = segment_signal[2] ^ signal_bitmask;
                break;
            }
        }

        let mut bitmask_069: u8 = 0;
        for j in 0..NUMBER_BASE {
            let signal = input[i + j];
            let signal_len = signal.len();
            if signal_len == DIGIT_SEGMENT_COUNT[0]
                || signal_len == DIGIT_SEGMENT_COUNT[6]
                || signal_len == DIGIT_SEGMENT_COUNT[9]
            {
                bitmask_069 ^= segments_to_bitmask(signal, None);
            }
        }
        bitmask_069 = !bitmask_069 ^ 128;

        segment_signal[3] &= bitmask_069;
        segment_signal[1] ^= segment_signal[3];
        bitmask_069 ^= segment_signal[3];

        segment_signal[2] &= bitmask_069;
        segment_signal[5] ^= segment_signal[2];
        bitmask_069 ^= segment_signal[2];

        segment_signal[4] = bitmask_069;
        segment_signal[6] = !(segment_signal[0]
            | segment_signal[1]
            | segment_signal[2]
            | segment_signal[3]
            | segment_signal[4]
            | segment_signal[5])
            ^ 128;

        // println!(
        //     "{:?}, {:#010b}",
        //     &input[i..(i + NUMBER_BASE)],
        //     segment_signal[6]
        // );

        let mut weights: [u8; SEGMENT_COUNT] = [0; SEGMENT_COUNT];
        for (j, segment) in segment_signal.iter().enumerate() {
            let mut k = 0;
            let mut index = *segment;
            while index > 1 {
                index /= 2;
                k += 1;
            }

            weights[k] = 2_u8.pow(j as u32);
        }

        let mut output_value: String = Default::default();
        for j in 0..OUTPUT_VALUE_COUNT {
            output_value.push(bitmask_to_digit(segments_to_bitmask(
                input[i + NUMBER_BASE + j],
                Some(&weights),
            )));
        }

        total += output_value.parse::<i32>().unwrap();
    }

    total
}

fn segments_to_bitmask(segments: &str, segment_weights: Option<&[u8; SEGMENT_COUNT]>) -> u8 {
    let weights = *segment_weights.unwrap_or(&[1, 2, 4, 8, 16, 32, 64]);
    let mut bitmask: u8 = 0;

    for segment in segments.chars() {
        match segment {
            'a' => bitmask += weights[0],
            'b' => bitmask += weights[1],
            'c' => bitmask += weights[2],
            'd' => bitmask += weights[3],
            'e' => bitmask += weights[4],
            'f' => bitmask += weights[5],
            'g' => bitmask += weights[6],
            _ => panic!(),
        }
    }

    bitmask
}

fn bitmask_to_digit(bitmask: u8) -> char {
    match bitmask {
        0b01110111 => '0',
        0b00100100 => '1',
        0b01011101 => '2',
        0b01101101 => '3',
        0b00101110 => '4',
        0b01101011 => '5',
        0b01111011 => '6',
        0b00100101 => '7',
        0b01111111 => '8',
        0b01101111 => '9',
        _ => {
            println!("{:#010b}", bitmask);
            panic!()
        }
    }
}

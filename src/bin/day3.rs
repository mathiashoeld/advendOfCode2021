use std::fs;

fn main() {
    let filename = "data/day3/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{}", power_consumption(&contents));
}

fn power_consumption(diagnostic_report: &str) -> i32 {
    let binaries = diagnostic_report.lines().collect::<Vec<&str>>();
    let gamma_str = most_common_bits(binaries.clone());
    let epsilon_str = least_common_bits(binaries.clone());
    let gamma = i32::from_str_radix(&gamma_str[..], 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_str[..], 2).unwrap();

    let mut oxygen_generator_list = binaries.clone();
    let mut co2_scrubber_list = binaries.clone();

    for i in 0..gamma_str.len() {
        let oxygen_str = most_common_bits(oxygen_generator_list.clone());
        let most_common_bit = oxygen_str.chars().nth(i).unwrap();

        oxygen_generator_list.retain(|&n| n.chars().nth(i).unwrap() == most_common_bit);

        println!("Most common bit: {}", most_common_bit);
        println!("{:?}", oxygen_generator_list);
        if oxygen_generator_list.len() == 1 {
            break;
        }
    }

    for i in 0..gamma_str.len() {
        let co2_str = least_common_bits(co2_scrubber_list.clone());
        let least_common_bit = co2_str.chars().nth(i).unwrap();

        co2_scrubber_list.retain(|&n| n.chars().nth(i).unwrap() == least_common_bit);

        println!("Least common bit: {}", least_common_bit);
        println!("{:?}", co2_scrubber_list);

        if co2_scrubber_list.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = i32::from_str_radix(&oxygen_generator_list[0], 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(&co2_scrubber_list[0], 2).unwrap();

    println!("{:?}", oxygen_generator_list);
    println!("oxy: {}", oxygen_generator_rating);
    println!("oxy: {}", co2_scrubber_rating);

    println!(
        "Part2 result: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );

    gamma * epsilon
}

fn count_bits(input_list: Vec<&str>) -> Vec<(i32, i32)> {
    let mut gamma_bits: Vec<(i32, i32)> = vec![];
    for x in 0..input_list[0].len() {
        gamma_bits.push(input_list.iter().map(|&n| n.chars().nth(x).unwrap()).fold(
            (0, 0),
            |acc, n| match n {
                '0' => (acc.0 + 1, acc.1),
                '1' => (acc.0, acc.1 + 1),
                a => {
                    println!("{}", a);
                    panic!()
                }
            },
        ));
    }
    gamma_bits
}
fn most_common_bits(input_list: Vec<&str>) -> String {
    count_bits(input_list)
        .iter()
        .map(|(a, b)| if a > b { return '0' } else { return '1' })
        .collect::<String>()
}

fn least_common_bits(input_list: Vec<&str>) -> String {
    count_bits(input_list)
        .iter()
        .map(|(a, b)| if a <= b { return '0' } else { return '1' })
        .collect::<String>()
}

#[test]
fn day3_input_test() {
    let diagnostic_example = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(power_consumption(diagnostic_example), 198)
}

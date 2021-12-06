use std::fs;

fn main() {
    let filename = "data/day1/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let measurements = contents
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut subsums: Vec<u32> = vec![];
    for n in 2..measurements.len() {
        subsums.push(measurements[n] + measurements[n - 1] + measurements[n - 2]);
    }

    let mut compare_to = subsums[0];
    let measurement_differences = subsums
        .iter()
        .map(|current_value| {
            if compare_to == *current_value {
                compare_to = *current_value;
                return String::from("N/A");
            } else if compare_to < *current_value {
                compare_to = *current_value;
                return String::from("inc");
            } else {
                compare_to = *current_value;
                return String::from("dec");
            }
        })
        .filter(|measurement| measurement == &String::from("inc"))
        .collect::<Vec<String>>()
        .len();

    println!("{}", measurement_differences);
}

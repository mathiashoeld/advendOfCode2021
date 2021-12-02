use std::fs;

fn main() {
    let filename = "data/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let measurements = contents
        .split("\n")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut subsums: Vec<u32> = vec![];
    for n in 2..measurements.len() {
        subsums.push(measurements[n] + measurements[n - 1] + measurements[n - 2]);
    }

    let mut a = subsums[0];
    let measurement_differences = subsums
        .iter()
        .map(|v| {
            if a == *v {
                a = *v;
                return String::from("N/A");
            } else if a < *v {
                a = *v;
                return String::from("inc");
            } else {
                a = *v;
                return String::from("dec");
            }
        })
        .filter(|l| l == &String::from("inc"))
        .collect::<Vec<String>>()
        .len();

    println!("{}", measurement_differences);
}

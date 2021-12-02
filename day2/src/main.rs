use std::fs;

#[derive(Debug)]
struct Position {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn forward(&mut self, amount: i32) {
        self.horizontal += amount;
        self.depth += self.aim * amount;
    }

    fn down(&mut self, amount: i32) {
        self.aim += amount;
    }

    fn up(&mut self, amount: i32) {
        self.aim -= amount;
    }

    fn result(&self) -> i32 {
        &self.depth * &self.horizontal
    }
}
fn main() {
    let mut final_position = Position {
        aim: 0,
        horizontal: 0,
        depth: 0,
    };

    let filename = "data/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut commands: Vec<Vec<&str>> = vec![];
    for l in lines {
        commands.push(l.split(" ").collect::<Vec<&str>>());
    }

    for command in commands {
        match command[0] {
            "forward" => final_position.forward(command[1].parse::<i32>().unwrap_or(0)),
            "down" => final_position.down(command[1].parse::<i32>().unwrap_or(0)),
            "up" => final_position.up(command[1].parse::<i32>().unwrap_or(0)),
            _ => (),
        }
    }

    println!("{}", final_position.result());
}

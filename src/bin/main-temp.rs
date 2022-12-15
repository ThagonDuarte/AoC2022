use std::fs;

fn main() {
    let filepath = "src/bin/day0x/input.txt";
    let input = fs::read_to_string(filepath).expect("Should have been able to read the file.");

    for elem in input.lines() {}
}

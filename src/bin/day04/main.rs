use std::{fs};

fn main() {
    let filepath: &str = "src/bin/day04/input.txt";
    let input: String = fs::read_to_string(filepath).expect("Should have been able to read the file.");
    let mut result = 0;

    for elem in input.lines() {
        let line = expand_to_range(elem);
        if line.0.contains(&line.1) || line.1.contains(&line.0) {
            result += 1;
            println!("{}, {}",line.0, line.1);
        } 
    }
    let line2 = expand_to_range("76-83,77-87");

    println!("{}, {}",line2.0, line2.1);
    println!("Task 1: {}",result);
}

fn expand_to_range (elem: &str) -> (String,String) {
    let v: Vec<&str> = elem.split_terminator(',').collect();
    let first: Vec<&str> = v[0].split_terminator('-').collect();
    let second: Vec<&str> = v[1].split_terminator('-').collect();
    let mut line: (String,String) = (String::new(),String::new()); 
    for i in first[0].parse::<usize>().unwrap()..(first[1].parse::<usize>().unwrap()+1) {
        line.0.push_str(&i.to_string());
        line.0.push(' ');
    }
    for i in second[0].parse::<usize>().unwrap()..(second[1].parse::<usize>().unwrap()+1) {
        line.1.push_str(&i.to_string());
        line.1.push(' ');
    }
    line
}
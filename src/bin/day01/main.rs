use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[2];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut most_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    println!("Searching for the most calories carried by one elf.");
    for elem in input.lines() {
        if elem != "" {
            let calories: i32 = elem.to_string().parse().unwrap();
            current_calories += calories;
        }
        if elem == "" {
            if current_calories > most_calories {
                most_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    println!(
        "The elf with the most calories has {} calories packed with him.",
        most_calories
    );

    println!("Searching for the sum of calories carried by the top three elves.");
    current_calories = 0;
    let mut elves: [i32; 2000] = [0; 2000];
    let mut i = 0;
    for elem in input.lines() {
        if elem != "" {
            let calories: i32 = elem.to_string().parse().unwrap();
            current_calories += calories;
        }
        if elem == "" {
            elves[i] = current_calories;
            i += 1;
            current_calories = 0;
        }
    }
    elves.sort();
    let len = elves.len();
    println!(
        "The sum of calories carried by the top three elves is {}.",
        elves[len - 1] + elves[len - 2] + elves[len - 3]
    );
}

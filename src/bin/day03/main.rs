use std::fs;

fn main() {
    let filepath = "src/bin/day03/input.txt";
    let input = fs::read_to_string(filepath).expect("Should have been able to read the file.");
    let mut sum_type: u32 = 0;
    let mut sum_group: u32 = 0;
    let mut counter: usize = 1;
    let mut group: [&str; 3] = ["", "", ""];
    for elem in input.lines() {
        sum_type += get_item_priority(get_shared_item(elem));
        group[counter - 1] = elem;
        if counter == 3 {
            sum_group += get_item_priority(get_group_type(group));
            counter = 0;
        }
        counter += 1;
    }
    println!("The sum of priorities of shared items is {}.", sum_type);
    println!("The sum of priorities of group badges is {}.", sum_group);
}

fn get_shared_item(items: &str) -> char {
    let (first, second): (&str, &str) = items.split_at(items.len() / 2);
    for i in 65u8..=122 {
        if first.contains(i as char) && second.contains(i as char) {
            return i as char;
        }
    }
    return '0';
}

fn get_group_type(group: [&str; 3]) -> char {
    for i in 65u8..=122 {
        if group[0].contains(i as char)
            && group[1].contains(i as char)
            && group[2].contains(i as char)
        {
            return i as char;
        }
    }
    return '0';
}

fn get_item_priority(item: char) -> u32 {
    let mut priority: u32 = 0;
    if item.is_lowercase() {
        priority = u32::from(item) - 96
    }
    if item.is_uppercase() {
        priority = u32::from(item) - 38
    }
    return priority;
}

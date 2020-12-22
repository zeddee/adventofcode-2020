use std::io::{self, prelude::*, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_2020() {
        assert_eq!(is_sum_2020(1010, 1010), true)
    }

    #[test]
    fn get_2020_product() {
        assert_eq!(get_product(5435345, 345345), 1877069219025)
    }
}

pub fn is_sum_2020(num1:u64, num2:u64) -> bool {
    num1 + num2 == 2020
}

pub fn get_product(num1: u64, num2: u64) -> u64 {
    num1 * num2
}

fn parse_number(line: String) -> u64 {
    line.trim().parse::<u64>().unwrap()
}

pub fn get_filepath(filename: &str) -> std::path::PathBuf {
    let filepath = std::env::current_dir()
        .expect("Invalid path to file")
        .join(std::path::Path::new(filename));

    if !filepath.is_file() {
        panic!("Cannot read file")
    }

    filepath
}

pub fn read_numlist_from_file(filepath: std::path::PathBuf) -> Vec<u64> {
    let file = std::fs::File::open(filepath)
        .unwrap();

    let lines = BufReader::new(file).lines();

    let mut list: Vec<u64> = Vec::new();

    
    for (_, line) in lines.enumerate() {
        let thisline = line.unwrap();

        for c in thisline.chars() {
            if !c.is_ascii_digit() {
                eprintln!("Invalid number: {}", &thisline)
            }
        }

        let resulting_line: u64 = thisline.parse().unwrap();
        
        list.push(resulting_line);
    }

    list
}

pub fn ask_2020() {
    let mut line1 = String::new();
    let mut line2 = String::new();
    
    println!("Enter your first number:");
    std::io::stdin().read_line(&mut line1).unwrap();

    println!("Enter your second number:");
    std::io::stdin().read_line(&mut line2).unwrap();

    println!("{}", is_sum_2020(parse_number(line1), parse_number(line2)));
}

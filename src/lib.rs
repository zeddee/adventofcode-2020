use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_2020() {
        assert_eq!(is_sum_2020(vec![1010, 1010]), true)
    }

    #[test]
    fn get_2020_product() {
        assert_eq!(get_product(vec![5435345, 345345]), 1877069219025)
    }
}

pub fn is_sum_2020(numlist: Vec<u64>) -> bool {
    let mut total: u64 = numlist[0];
    for i in 1..numlist.len() {
        total = total + numlist[i];
    }
    total == 2020
}

pub fn get_product(numlist: Vec<u64>) -> u64 {
    let mut product: u64 = numlist[0];
    for i in 1..numlist.len() {
        if i <= numlist.len() {
            product = product * numlist[i]; 
        }
    }
    product
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

    println!("{}", is_sum_2020(vec![parse_number(line1), parse_number(line2)]));
}


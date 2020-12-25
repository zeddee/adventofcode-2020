use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_2020() {
        assert_eq!(Utils::is_sum_2020(vec![1010, 1010]), true)
    }

    #[test]
    fn get_2020_product() {
        assert_eq!(Utils::get_product(vec![5435345, 345345]), 1877069219025)
    }

    #[test]
    fn password_set_parse() {
        assert_eq!(
            PasswordRecord::new("1-3 a: abcde"),
            PasswordRecord{ rule: PasswordRule { c: 'a', count_min: 1, count_max: 3 }, password: String::from("abcde")},
        );
        assert_eq!(
            PasswordRecord::new("1-3 b: cdefg"),
            PasswordRecord{ rule: PasswordRule { c: 'b', count_min: 1, count_max: 3 }, password: String::from("cdefg")},
        );
        assert_eq!(
            PasswordRecord::new("2-9 c: ccccccccc"),
            PasswordRecord{ rule: PasswordRule { c: 'c', count_min: 2, count_max: 9 }, password: String::from("ccccccccc")},
        );
    }

    #[test]
    fn password_set_validate() {
        assert_eq!(
            PasswordRecord::new("1-3 a: abcde").validate_password(),
            true
        );
        assert_eq!(
            PasswordRecord::new("1-3 b: cdefg").validate_password(),
            false,
        );
        assert_eq!(
            PasswordRecord::new("2-9 c: ccccccccc").validate_password(),
            true,
        );
    }
}


#[derive(Debug)]
pub struct PuzzleInput {
    content: Vec<String>
}

impl PuzzleInput {
    pub fn from_file(filepath: std::path::PathBuf) -> Self {
        let file = std::fs::File::open(filepath)
        .unwrap();
        
        let lines = BufReader::new(file).lines();
        
        let mut list: Vec<String> = Vec::new();
        
        
        for (_, line) in lines.enumerate() {        
            list.push(line.unwrap());
        }
        
        return Self {content: list}
    }
    
    pub fn parse_content_to_numbers(filepath: std::path::PathBuf) -> Vec<u64> {
        let content = &Self::from_file(filepath);
        let mut list: Vec<u64> = Vec::new();
        
        for line in &content.content {
            
            for c in line.chars() {
                if !c.is_ascii_digit() {
                    eprintln!("Invalid number: {}", &line)
                }
            }
            
            let resulting_line: u64 = line.parse().unwrap();
            
            list.push(resulting_line);
        }
        
        list
    }
    
    pub fn check_valid_passwords(filepath: std::path::PathBuf) -> u32 {

        let input = &Self::from_file(filepath);

        let mut valid_password_count = 0;

        for line in &input.content {
            let record = PasswordRecord::new(line);

            if record.validate_password() {
                valid_password_count += 1;
            }
        }
        
        valid_password_count
    }
}

#[derive(Debug, PartialEq)]
struct PasswordRecord {
    rule: PasswordRule,
    password: String,
}

impl PasswordRecord {
    fn new(record: &str) -> Self {
        let password_set: Vec<&str> = record.split(':').collect();

        Self {
            rule: PasswordRule::parse_rule(password_set[0]),
            password: password_set[1].trim().to_owned(),
        }
    }
    fn validate_password(&self) -> bool {
        let matched: Vec<&str> = self.password.rmatches(self.rule.c).collect();
        let matches_count = matched.len();
        let matches_count_u32 = matches_count as u32;

        self.rule.count_max >= matches_count_u32
            && matches_count_u32 >= self.rule.count_min
    }
}

#[derive(Debug, PartialEq)]
struct PasswordRule {
    c: char,
    count_min: u32,
    count_max: u32,
}

impl PasswordRule {
    fn parse_rule(rule: &str) -> Self {
        let level1: Vec<&str> = rule.split(' ').collect();
        let level2: Vec<&str> = level1[0].split('-').collect();
        let level2_nums: Vec<u32> = vec![level2[0].parse().unwrap(), level2[1].parse().unwrap()];

        Self {
            c: level1[1].chars().last().unwrap(),
            count_min: level2_nums[0],
            count_max: level2_nums[1],
        }
    }
}

pub struct Utils;

impl Utils {

    pub fn get_filepath(filename: &str) -> std::path::PathBuf {
        let filepath = std::env::current_dir()
            .expect("Invalid path to file")
            .join(std::path::Path::new(filename));

        if !filepath.is_file() {
            panic!("Cannot read file")
        }

        filepath
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

    pub fn ask_2020() {
        let mut line1 = String::new();
        let mut line2 = String::new();
        
        println!("Enter your first number:");
        std::io::stdin().read_line(&mut line1).unwrap();

        println!("Enter your second number:");
        std::io::stdin().read_line(&mut line2).unwrap();

        println!("{}", Utils::is_sum_2020(vec![Utils::parse_number(line1), Utils::parse_number(line2)]));
    }
}



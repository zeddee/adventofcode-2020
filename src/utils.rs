

pub fn get_filepath(filename: &str) -> std::path::PathBuf {
    let filepath = std::env::current_dir()
        .expect("Invalid path to file")
        .join(std::path::Path::new(filename));

    if !filepath.is_file() {
        panic!("Cannot read file: {:?}", filepath)
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

    println!("{}", is_sum_2020(vec![parse_number(line1), parse_number(line2)]));
}

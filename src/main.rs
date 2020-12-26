mod utils;
mod day01;

fn day1() -> u64 {
    let filename = "src/data/01.csv";
    let numlist = day01::PuzzleInput::parse_content_to_numbers(utils::get_filepath(filename));
    
    let mut result: u64 = 0;

    for idx in 1..numlist.len() {
        if idx == numlist.len() {
            break;
        }

        let num1 = numlist[idx-1];

        for i in 1..numlist.len() {
            let num2 = numlist[i];

            for i in 1..numlist.len() {
                let num3 = numlist[i];

                if utils::is_sum_2020(vec![num1, num2, num3]) {
                    result = utils::get_product(vec![num1, num2, num3])
                }
            }
        }
    }
    
    return result
}

fn day2() -> u32 {
    let filename = "src/data/02.txt";
    day01::PuzzleInput::check_valid_passwords(utils::get_filepath(filename))
}

fn day2_part2() -> u32 {
    let filename = "src/data/02.txt";
    day01::PuzzleInput::check_valid_passwords_awkward(utils::get_filepath(filename))
}

fn main() {
    println!("{}", day1());
    println!("Valid passwords: {}", day2());
    println!("Valid (awkward) passwords: {}", day2_part2());
}


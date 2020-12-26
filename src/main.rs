mod utils;
mod day02;

fn main() {
    println!("{}", day02::day1());
    println!("Valid passwords: {}", day02::day2());
    println!("Valid (awkward) passwords: {}", day02::day2_part2());
}


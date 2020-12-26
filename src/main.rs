mod utils;
mod day02;
mod day03;

fn main() {
    println!("{}", day02::day1());
    println!("Valid passwords: {}", day02::day2());
    println!("Valid (awkward) passwords: {}", day02::day2_part2());
    day03::day3()
}


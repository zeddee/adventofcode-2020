use ac::*;

fn day1() -> u64 {
    let filename = "src/data/01.csv";
    let numlist = PuzzleInput::parse_content_to_numbers(Utils::get_filepath(filename));
    
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

                if Utils::is_sum_2020(vec![num1, num2, num3]) {
                    result = Utils::get_product(vec![num1, num2, num3])
                }
            }
        }
    }
    
    return result
}

fn main() {
    println!("{}", day1());
}


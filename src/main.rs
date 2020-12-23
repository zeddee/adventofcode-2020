use ac::*;

fn day1() -> u64 {
    //ac::ask_2020();
    let filename = "src/data/01.csv";
    //std::io::stdin().read_line(&mut filename).unwrap();
    let list_of_strings = read_numlist_from_file(get_filepath(filename));
    
    let mut result: u64 = 0;

    for idx in 1..list_of_strings.len() {
        if idx == list_of_strings.len() {
            break;
        }

        let num1 = list_of_strings[idx-1];

        for i in 1..list_of_strings.len() {
            let num2 = list_of_strings[i];

            for i in 1..list_of_strings.len() {
                let num3 = list_of_strings[i];

                if is_sum_2020(vec![num1, num2, num3]) {
                    result = get_product(vec![num1, num2, num3])
                }
            }
        }
    }
    
    return result
}

fn main() {
    println!("{}", day1());
}


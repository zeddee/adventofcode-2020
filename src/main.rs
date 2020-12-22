use ac::*;

fn main() {
    //ac::ask_2020();
    let filename = "src/data/01.csv";
    //std::io::stdin().read_line(&mut filename).unwrap();
    let list_of_strings = read_numlist_from_file(get_filepath(filename));
    
    for idx in 1..list_of_strings.len() {
        if idx == list_of_strings.len() {
            break;
        }

        let num1 = list_of_strings[idx-1];

        for i in 1..list_of_strings.len() {
            let num2 = list_of_strings[i];

            if is_sum_2020(num1, num2) {
                println!("Is sum 2020?: {}", is_sum_2020(num1, num2));
                println!("{}", get_product(num1, num2))            
            }
        }
    }
}


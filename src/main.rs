use ac::*;

fn main() {
    //ac::ask_2020();
    let filename = "src/data/01.csv";
    //std::io::stdin().read_line(&mut filename).unwrap();
    let list_of_strings = parse_listfile(filename);
    println!("{:?}", list_of_strings);
}

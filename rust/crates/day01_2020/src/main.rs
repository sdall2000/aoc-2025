use utils::read_numbers_from_file;

fn main() {
    let path = "../../../puzzle_input/day_01_2020.txt";
    let values = read_numbers_from_file(path)
        .expect("Failed to read numbers from puzzle input");

    println!("Read {} values from puzzle input", values.len());
    println!("First 5 values: {:?}", &values[..5.min(values.len())]);
}

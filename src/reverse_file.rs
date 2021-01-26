use fire::reverse::reverse_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Wrong num of params");
        return;
    }
    let input_filepath = args[1].clone();
    let output_filepath = format!("{}.reversed", input_filepath).to_string();

    reverse_file(&input_filepath, &output_filepath);
}

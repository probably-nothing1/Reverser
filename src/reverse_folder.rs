use fire::reverse::reverse_file;
use glob::{glob, Paths};
use rayon::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Wrong num of params");
        return;
    }

    let folder_path = args[1].clone();
    reverse_folder(&folder_path);
}

pub fn reverse_folder(folder_path: &String) {
    let file_paths = get_data_file_paths(folder_path);
    let file_pairs = get_filename_pairs(file_paths);

    file_pairs
        .par_iter()
        .for_each(|(input, output)| reverse_file(&input, &output));
}

fn get_data_file_paths(folder_path: &String) -> Paths {
    let pattern = format!("{}/*.data", folder_path);
    glob(pattern.as_str()).expect("Failed to glob")
}

fn get_filename_pairs(file_paths: Paths) -> Vec<(String, String)> {
    file_paths
        .map(|filename| {
            let path = filename.expect("Glob error parsing filename");
            let input_file_path = path.to_str().unwrap().to_string();
            let output_file_path = format!("{}.reversed", input_file_path);
            (input_file_path, output_file_path)
        })
        .collect::<Vec<(String, String)>>()
}

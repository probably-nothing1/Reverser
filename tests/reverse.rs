#[cfg(test)]
mod tests {
    use fire::reverse::reverse_file;
    use std::fs;

    #[test]
    fn correctness_test() {
        let tests_cases = vec!["task_example", "small_generated"];
        for test_case in tests_cases {
            let input_filepath = format!("./tests/correctness/{}.data", test_case).to_string();
            let output_filepath =
                format!("./tests/correctness/{}.data.reversed", test_case).to_string();
            let solution_filepath =
                format!("./tests/correctness/{}.data.solution", test_case).to_string();

            reverse_file(&input_filepath, &output_filepath);
            let result = fs::read_to_string(&output_filepath).expect("Unable to read result file");
            let solution =
                fs::read_to_string(&solution_filepath).expect("Unable to read solution file");

            assert_eq!(result, solution);

            fs::remove_file(output_filepath).expect("Failed to remove solution file");
        }
    }

    #[test]
    fn performance_test() {
        let tests_cases = vec![
            "small_10mb",
            "small_50mb",
            "medium_200mb",
            "medium_500mb",
            "large_1gb",
        ];
        for test_case in tests_cases {
            let input_filepath = format!("./tests/performance/{}.data", test_case).to_string();
            let output_filepath =
                format!("./tests/performance/{}.data.reversed", test_case).to_string();
            let solution_filepath =
                format!("./tests/performance/{}.data.solution", test_case).to_string();

            reverse_file(&input_filepath, &output_filepath);
            let result = fs::read_to_string(&output_filepath).expect("Unable to read result file");
            let solution =
                fs::read_to_string(&solution_filepath).expect("Unable to read solution file");

            assert_eq!(result, solution);

            fs::remove_file(output_filepath).expect("Failed to remove solution file");
        }
    }
}

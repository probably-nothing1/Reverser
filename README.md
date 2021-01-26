# Reverser

## Description

Program takes directory path as input. For every file (*.data) in the directory, reverses both each file and lines in a file.


## Example
input of `file.data`
```
  this is a haiku
  the number of syllables
  is palindromic
```

output of `file.data.reserved`
```
  cimordnilap si
  selballys fo rebmun eht
  ukiah a si siht
```

## Installation and running
You need to have Rust installed on your machine.

To build the project type:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

To run it
```bash
./target/release/reverse_folder your_folder_with_files
```

## Generate test files
### Generate signle file pair (input file and solution file)
```bash
python utils/gen.py ./tests/performance/medium_200mb.data 2000000 100
```

### Generate entire folder of input and solution files
```bash
python utils/gen_folders.py ./tests/large_folder_10mb_files 100 100000 100
```


## Testing
```bash
cargo test
```

## Benchmark
You need to have benchmarking tool `hyperfine` installed to run benchmarks.

### Benchmark folders
```bash
bash benchmark/bench.sh
```

### Benchmark single files
You can also benchmark single files. When compiling you get extra executable file (`./target/release/reverse_file`) that reverses signgle file.
```bash
bash benchmark/bench_files.sh
```

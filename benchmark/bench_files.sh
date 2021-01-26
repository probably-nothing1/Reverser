#! /bin/bash

RUSTFLAGS="-C target-cpu=native" cargo build --release

files=(
    "performance/small_10mb"
    "performance/small_50mb"
    "performance/medium_200mb"
    "performance/medium_500mb"
    "performance/large_1gb"
)
for file in "${files[@]}"; do
    input="./tests/$file.data"
    echo "$input"

    hyperfine "./target/release/reverse_file $input" \
        --runs 5 \
        --warmup 3 \
        --style "full"
        # --export-json "results.json"
done

echo

echo "Checking correctness"
for file in "${files[@]}"; do
    solution="./tests/$file.data.solution"
    output="./tests/$file.data.reversed"

    if cmp -s "$output" "$solution"; then
        printf "$file OK\n"
    else
        printf "$file FAIL\n"
        exit 1
    fi
    rm "$output"
done

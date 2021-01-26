#! /bin/bash

RUSTFLAGS="-C target-cpu=native" cargo build --release

folders=(
    "correctness"
    "performance"
    "small_folder_small_files"
    "large_folder_10mb_files"
)

for folder in "${folders[@]}"; do
    input="./tests/$folder"
    echo "$input"

    hyperfine "./target/release/reverse_folder $input" \
        --runs 5 \
        --warmup 3 \
        --style "full"
        # --export-json "results.json"
done

echo

for folder in "${folders[@]}"; do
    for file in ./tests/$folder/*.data; do
        solution="$file.solution"
        output="$file.reversed"

        if ! cmp -s "$output" "$solution"; then
            printf "FAIL $file \n"
            exit 1
        fi

        rm "$output"
    done
    printf "OK $folder\n"
done

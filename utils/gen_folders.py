import os
import random
import string
import sys

# from utils.gen import create_file


def create_file(path, rows, cols):
    lines = []
    with open(path, "w+") as f:
        for i in range(rows):
            letters = string.ascii_lowercase
            line = "".join(random.choice(letters) for _ in range(cols))
            if i + 1 == rows:
                f.write(line)
            else:
                f.write(line + "\n")

            lines.append(line)

    solution_path = f"{path}.solution"
    with open(solution_path, "w+") as f:
        for i, line in enumerate(lines[::-1]):
            if i + 1 == len(lines):
                f.write(line[::-1])
            else:
                f.write(line[::-1] + "\n")


def create_folder(path, num_files, rows, cols):
    if os.path.exists(path):
        raise Exception(f"Directory/file {path} already exists")

    os.makedirs(path)
    for i in range(num_files):
        filename = os.path.join(path, f"file_{i}.data")
        create_file(filename, rows, cols)


if __name__ == "__main__":
    import sys

    if len(sys.argv) != 5:
        print("Wrong number of args")

    path, num_files, rows, cols = (
        sys.argv[1],
        int(sys.argv[2]),
        int(sys.argv[3]),
        int(sys.argv[4]),
    )
    create_folder(path, num_files, rows, cols)

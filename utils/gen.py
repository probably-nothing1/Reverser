import random
import string
import sys


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


if __name__ == "__main__":
    import sys

    if len(sys.argv) != 4:
        print("Wrong number of args")
    path, rows, cols = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    create_file(path, rows, cols)

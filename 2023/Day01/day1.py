import re

with open("input.txt") as infile:
    lines = infile.readlines()
    numbers = []
    for line in lines:
        line_numbers = re.sub(r"[a-zA-Z]", "", line.strip())
        numbers.append(int(line_numbers[0] + line_numbers[-1]))
        print(line, numbers[-1])
    print(sum(numbers))

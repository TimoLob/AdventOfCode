import re


def string_to_number(input):
    translator = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    if input in translator:
        return translator[input]
    return int(input)


with open("input.txt") as infile:
    lines = infile.readlines()
    numbers = []
    for line in lines:
        line_numbers = re.findall(
            r"(?=([0-9]|one|two|three|four|five|six|seven|eight|nine))", line.strip()
        )
        print(line_numbers)
        numbers.append(
            string_to_number(line_numbers[0]) * 10 + string_to_number(line_numbers[-1])
        )
    print(sum(numbers))

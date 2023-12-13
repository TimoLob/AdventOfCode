
import functools
from typing import Tuple

@functools.cache
def find_permutations(remaining_input : str, remaining_pattern:Tuple[int, ...], current_defects):
    # print(remaining_input,remaining_pattern,current_defects)
    if len(remaining_input)==0:
        if len(remaining_pattern) == 0:
            return 1
        elif len(remaining_pattern) == 1:
            return 1 if current_defects==remaining_pattern[0] else 0
        else:
            return 0
    if len(remaining_pattern) == 0:
        return 0 if '#' in remaining_input else 1

    first = remaining_input[0]
    current_pattern = remaining_pattern[0]
    if first == '?':
        if current_defects==0:
            return find_permutations(remaining_input[1:],remaining_pattern,0) + find_permutations(remaining_input[1:],remaining_pattern,1)
        if current_defects == current_pattern:
            return find_permutations(remaining_input[1:],remaining_pattern[1:],0)
        return find_permutations(remaining_input[1:],remaining_pattern,current_defects+1)
    
    if first == '.':
        if current_defects==0:
            return find_permutations(remaining_input[1:],remaining_pattern,0)
        if current_defects == current_pattern:
            return find_permutations(remaining_input[1:],remaining_pattern[1:],0)
        else:
            return 0
        
    if first == '#':
        return find_permutations(remaining_input[1:],remaining_pattern,current_defects+1)

# ???.### 1,1,3
def prepare_line(line:str):
    line = line.strip()
    input, pattern = line.split(" ")
    pattern = [int(x) for x in pattern.split(',')]
    #return input,tuple(pattern) # Task 1
    return ((input+'?')*5)[:-1],tuple(pattern*5)


def solve():
    with open("input.txt") as input:
        lines = input.readlines()
        total = 0
        for line in lines:
            input,pattern = prepare_line(line)
            perms = find_permutations(input,pattern,0)
            # print(line,"=>",perms)
            total+= perms
        print(total)

solve()





def find_permutations(remaining_input : str, remaining_pattern, current_defects):
    if len(remaining_input) == 0:
        if len(remaining_pattern) > 0:
            if len(remaining_pattern) == 1 and remaining_pattern[0]==current_defects:
                return 1
            return 0
        return 1
    if len(remaining_pattern) == 0:
        return 0 if '#' in remaining_input else 1
    first = remaining_input[0]
    if first == '.':
        if current_defects>0:
            if current_defects != remaining_pattern[0]:
                return 0
            return find_permutations(remaining_input[remaining_pattern[0]+1:],remaining_pattern[1:],0)
        return find_permutations(remaining_input[1:],remaining_pattern,0)
    if first=='#':
        current_defects+=1
        return find_permutations(remaining_input[1:],remaining_pattern,current_defects)
    if first == '?':
        if current_defects==0:
            return find_permutations(remaining_input[1:],remaining_pattern,0) + find_permutations(remaining_input[1:],remaining_pattern,1)
        else:
            if current_defects==remaining_pattern[0]:
                return find_permutations(remaining_input[1:],remaining_pattern[1:],0)
            else:
                return find_permutations(remaining_input[1:],remaining_pattern,current_defects+1)
    print("I shouldn't be here",remaining_input,remaining_pattern,current_defects,first)


with open("example.txt") as input:
    lines = input.readlines()
    total = 0
    for line in lines:
        # ???????##?????#?#? 9,6
        input, pattern = line.split(" ")
        pattern = [int(x) for x in pattern.split(',')]
        total+= find_permutations(input,pattern,0)
    print(total)


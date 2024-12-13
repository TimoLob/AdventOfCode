from z3 import *
import re

class Vector:
    def __init__(self,x,y):
        self.x=x
        self.y=y
    def __repr__(self):
        return f"(x={self.x},y={self.y})"


class Machine:
    def __init__(self,buttonA,buttonB,target):
        self.buttonA = buttonA
        self.buttonB = buttonB
        self.target = target


    def __repr__(self):
        return f"A : {self.buttonA}, B : {self.buttonB}, Target : {self.target}"

    def optimize(self):
        o = Optimize()
        num_a = Int("a")
        num_b = Int("b")
        cost = 3 * num_a + 1 * num_b

        o.add([
            self.target.x == self.buttonA.x * num_a + self.buttonB.x * num_b,
            self.target.y == self.buttonA.y * num_a + self.buttonB.y * num_b
        ])
        o.minimize(cost)
        if o.check():
            m = o.model()
            if m[num_a] != None:
                return m[num_b].as_long() + 3*m[num_a].as_long() 
        return 0


def parse(input : str):
    machines = input.split("\n\n")
    result = []
    for machine in machines:
        machine = machine.strip()
        ba,bb,target = machine.split('\n')
        button_regex = "^.*X\\+([0-9]+), Y\\+([0-9]+)"
        ba = re.search(button_regex,ba)
        ba = Vector(int(ba.group(1)),int(ba.group(2)))
        bb = re.search(button_regex,bb)
        bb = Vector(int(bb.group(1)),int(bb.group(2)))

        target_regex = "^.*X=([0-9]+), Y=([0-9]+)"
        target = re.search(target_regex,target)
        target = Vector(int(target.group(1)),int(target.group(2)))

        result.append(Machine(ba,bb,target))


    return result

def part1(filename):
    with open(filename) as infile:
        input = infile.read()
    machines = parse(input)


    total = 0
    for m in machines:
        total += m.optimize()


    return total

assert(part1("example.txt") == 480)

def part2(filename):
    with open(filename) as infile:
        input = infile.read()
    machines = parse(input)


    total = 0
    for m in machines:
        m.target.x += 10000000000000
        m.target.y += 10000000000000
        total += m.optimize()


    return total
print("Part1:",part1("input.txt"))
print("Part2:",part2("input.txt"))


